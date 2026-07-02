#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
from tempfile import TemporaryDirectory

import cadquery as cq
from cadquery import exporters


OUT_DIR = Path(__file__).resolve().parent

# ESP32-C3-DevKit-RUST-1 v1.2a, dimensions extracted from the KiCad PCB.
BOARD_W = 22.86
BOARD_L = 63.50
BOARD_T = 1.60
BOARD_CORNER_R = 0.508
MOUNT_HOLE_D = 3.048
MOUNT_HOLE_X = [2.540, 20.320]
MOUNT_HOLE_Y = [2.540, 60.960]

# Working placeholder for a flat LiPo cell. Change these when you choose the cell.
BATTERY_W = 30.0
BATTERY_L = 50.0
BATTERY_H = 8.0

# Print and case parameters.
WALL = 1.8
FLOOR = 1.6
BOARD_CLEARANCE = 0.8
BATTERY_CLEARANCE = 2.0
DIVIDER_T = 1.4
INNER_H = 12.0
CASE_R = 3.0

STANDOFF_H = 3.0
STANDOFF_D = 5.6
STANDOFF_PILOT_D = 2.2

LID_T = 1.8
LID_LIP_H = 2.0
LID_LIP_CLEARANCE = 0.45
LID_SCREW_CLEARANCE_D = 2.8
LID_COUNTERBORE_D = 5.2
LID_COUNTERBORE_H = 1.0

USB_CUT_W = 12.0
USB_CUT_H = 7.0
USB_CUT_Z = FLOOR + STANDOFF_H + BOARD_T + 1.7

BUTTON_ACCESS_D = 4.2


BOARD_BAY_W = BOARD_W + 2 * BOARD_CLEARANCE
BATTERY_BAY_W = BATTERY_W + 2 * BATTERY_CLEARANCE
INNER_W = BOARD_BAY_W + DIVIDER_T + BATTERY_BAY_W
INNER_L = max(BOARD_L + 2 * BOARD_CLEARANCE, BATTERY_L + 2 * BATTERY_CLEARANCE)
OUTER_W = INNER_W + 2 * WALL
OUTER_L = INNER_L + 2 * WALL
BASE_H = FLOOR + INNER_H

INNER_X0 = -INNER_W / 2
BOARD_X = INNER_X0 + BOARD_BAY_W / 2
DIVIDER_X = INNER_X0 + BOARD_BAY_W + DIVIDER_T / 2
BATTERY_X = INNER_X0 + BOARD_BAY_W + DIVIDER_T + BATTERY_BAY_W / 2

# Footprint positions converted from KiCad board coordinates to the centered case coordinates.
SWITCH_LOCAL = [(-5.4864, 0.762), (5.5880, 0.762)]
USB_LOCAL_Y = 125.984 - (65.024 + BOARD_L / 2)


def box(x: float, y: float, z: float) -> cq.Workplane:
    return cq.Workplane("XY").box(x, y, z, centered=(True, True, False))


def rounded_box(x: float, y: float, z: float, radius: float) -> cq.Workplane:
    solid = box(x, y, z)
    if radius > 0:
        solid = solid.edges("|Z").fillet(min(radius, x / 2 - 0.2, y / 2 - 0.2))
    return solid


def cyl_at(x: float, y: float, diameter: float, height: float, z: float = 0.0) -> cq.Workplane:
    return (
        cq.Workplane("XY")
        .center(x, y)
        .circle(diameter / 2)
        .extrude(height)
        .translate((0, 0, z))
    )


def mount_points() -> list[tuple[float, float]]:
    points: list[tuple[float, float]] = []
    for x in MOUNT_HOLE_X:
        for y in MOUNT_HOLE_Y:
            px = BOARD_X - BOARD_W / 2 + x
            py = -BOARD_L / 2 + y
            points.append((px, py))
    return points


def make_base() -> cq.Workplane:
    outer = rounded_box(OUTER_W, OUTER_L, BASE_H, CASE_R)
    cavity = box(INNER_W, INNER_L, INNER_H + 0.4).translate((0, 0, FLOOR))
    base = outer.cut(cavity)

    # Low center divider keeps the battery bay separate without fully hiding the board side.
    divider = box(DIVIDER_T, INNER_L - 1.0, 6.0).translate((DIVIDER_X, 0, FLOOR))
    base = base.union(divider)

    for x, y in mount_points():
        boss = cyl_at(x, y, STANDOFF_D, STANDOFF_H, FLOOR)
        pilot = cyl_at(x, y, STANDOFF_PILOT_D, STANDOFF_H + 0.4, FLOOR + 0.2)
        base = base.union(boss).cut(pilot)

    rail_t = 1.0
    rail_h = 2.0
    rail_l = BATTERY_L + 4.0
    rail_offset_x = BATTERY_W / 2 + BATTERY_CLEARANCE / 2
    for sx in (-1, 1):
        base = base.union(
            box(rail_t, rail_l, rail_h).translate(
                (BATTERY_X + sx * rail_offset_x, 0, FLOOR)
            )
        )
    for sy in (-1, 1):
        base = base.union(
            box(BATTERY_W + 2.0, rail_t, rail_h).translate(
                (BATTERY_X, sy * (BATTERY_L / 2 + rail_t / 2), FLOOR)
            )
        )

    usb_cut = box(USB_CUT_W, WALL + 3.0, USB_CUT_H).translate(
        (BOARD_X, OUTER_L / 2 - (WALL + 3.0) / 2, USB_CUT_Z - USB_CUT_H / 2)
    )
    base = base.cut(usb_cut)

    return base


def make_lid() -> cq.Workplane:
    top = rounded_box(OUTER_W, OUTER_L, LID_T, CASE_R).translate((0, 0, LID_LIP_H))
    lip = rounded_box(
        INNER_W - LID_LIP_CLEARANCE,
        INNER_L - LID_LIP_CLEARANCE,
        LID_LIP_H,
        max(0.8, CASE_R - WALL),
    )
    lid = top.union(lip)
    total_h = LID_LIP_H + LID_T

    for x, y in mount_points():
        through = cyl_at(x, y, LID_SCREW_CLEARANCE_D, total_h + 0.6, -0.3)
        counterbore = cyl_at(
            x,
            y,
            LID_COUNTERBORE_D,
            LID_COUNTERBORE_H + 0.2,
            total_h - LID_COUNTERBORE_H,
        )
        lid = lid.cut(through).cut(counterbore)

    for lx, ly in SWITCH_LOCAL:
        lid = lid.cut(cyl_at(BOARD_X + lx, ly, BUTTON_ACCESS_D, total_h + 0.6, -0.3))

    usb_notch = box(USB_CUT_W + 1.5, WALL + 6.0, total_h + 0.6).translate(
        (BOARD_X, OUTER_L / 2 - (WALL + 6.0) / 2, -0.3)
    )
    lid = lid.cut(usb_notch)

    return lid


def make_board_mock() -> cq.Workplane:
    board_z = FLOOR + STANDOFF_H
    board = rounded_box(BOARD_W, BOARD_L, BOARD_T, BOARD_CORNER_R).translate(
        (BOARD_X, 0, board_z)
    )
    for x, y in mount_points():
        board = board.cut(cyl_at(x, y, MOUNT_HOLE_D, BOARD_T + 0.4, board_z - 0.2))

    usb = box(9.8, 7.8, 3.3).translate(
        (BOARD_X, USB_LOCAL_Y - 1.1, board_z + BOARD_T)
    )
    module = box(13.2, 16.6, 2.4).translate((BOARD_X, -14.0, board_z + BOARD_T))
    button_bodies = cq.Workplane("XY")
    for lx, ly in SWITCH_LOCAL:
        button_bodies = button_bodies.union(
            box(4.3, 3.4, 1.8).translate((BOARD_X + lx, ly, board_z + BOARD_T))
        )
    return board.union(usb).union(module).union(button_bodies)


def make_battery_mock() -> cq.Workplane:
    return box(BATTERY_W, BATTERY_L, BATTERY_H).translate((BATTERY_X, 0, FLOOR))


def export_model(name: str, model: cq.Workplane | cq.Shape) -> None:
    exporters.export(model, str(OUT_DIR / f"{name}.step"))
    exporters.export(model, str(OUT_DIR / f"{name}.stl"))


def render_preview(objects: list[tuple[str, cq.Workplane, tuple[float, float, float], float]]) -> None:
    import vtk

    with TemporaryDirectory() as tmp:
        tmp_dir = Path(tmp)
        renderer = vtk.vtkRenderer()
        renderer.SetBackground(0.97, 0.97, 0.95)

        for name, model, color, opacity in objects:
            path = tmp_dir / f"{name}.stl"
            exporters.export(model, str(path))
            reader = vtk.vtkSTLReader()
            reader.SetFileName(str(path))
            mapper = vtk.vtkPolyDataMapper()
            mapper.SetInputConnection(reader.GetOutputPort())
            actor = vtk.vtkActor()
            actor.SetMapper(mapper)
            actor.GetProperty().SetColor(*color)
            actor.GetProperty().SetOpacity(opacity)
            actor.GetProperty().SetSpecular(0.18)
            actor.GetProperty().SetSpecularPower(18)
            renderer.AddActor(actor)

        window = vtk.vtkRenderWindow()
        window.SetOffScreenRendering(1)
        window.SetSize(1600, 1100)
        window.AddRenderer(renderer)

        camera = renderer.GetActiveCamera()
        camera.SetPosition(90, -105, 72)
        camera.SetFocalPoint(0, 0, 7)
        camera.SetViewUp(0, 0, 1)
        renderer.ResetCameraClippingRange()

        window.Render()

        image_filter = vtk.vtkWindowToImageFilter()
        image_filter.SetInput(window)
        image_filter.Update()

        writer = vtk.vtkPNGWriter()
        writer.SetFileName(str(OUT_DIR / "esp32_c3_devkit_rust_case_preview.png"))
        writer.SetInputConnection(image_filter.GetOutputPort())
        writer.Write()


def main() -> None:
    base = make_base()
    lid = make_lid()
    board_mock = make_board_mock()
    battery_mock = make_battery_mock()

    lid_preview = lid.translate((0, 0, BASE_H - LID_LIP_H + 6.0))

    export_model("esp32_c3_devkit_rust_case_base", base)
    export_model("esp32_c3_devkit_rust_case_lid", lid)

    preview_compound = cq.Compound.makeCompound(
        [base.val(), lid_preview.val(), board_mock.val(), battery_mock.val()]
    )
    export_model("esp32_c3_devkit_rust_case_fit_preview", preview_compound)

    render_preview(
        [
            ("base", base, (0.08, 0.09, 0.10), 1.0),
            ("lid", lid_preview, (0.18, 0.40, 0.74), 0.72),
            ("board", board_mock, (0.02, 0.48, 0.22), 1.0),
            ("battery", battery_mock, (0.78, 0.72, 0.18), 1.0),
        ]
    )

    print(f"Generated in: {OUT_DIR}")
    print(f"Outer size: {OUTER_W:.1f} x {OUTER_L:.1f} x {BASE_H + LID_T:.1f} mm")
    print(f"Battery placeholder: {BATTERY_W:.1f} x {BATTERY_L:.1f} x {BATTERY_H:.1f} mm")


if __name__ == "__main__":
    main()
