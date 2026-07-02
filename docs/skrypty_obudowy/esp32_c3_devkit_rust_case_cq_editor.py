from pathlib import Path
import sys


THIS_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(THIS_DIR))

from esp32_c3_devkit_rust_case import (  # noqa: E402
    BASE_H,
    LID_LIP_H,
    make_base,
    make_battery_mock,
    make_board_mock,
    make_lid,
)


base = make_base()
lid = make_lid().translate((0, 0, BASE_H - LID_LIP_H + 6.0))
board = make_board_mock()
battery = make_battery_mock()

show_object(base, name="01 podstawa", options={"color": "#111111", "alpha": 0.05})
show_object(board, name="02 makieta plytki", options={"color": "#1b9a4a", "alpha": 0.15})
show_object(battery, name="03 makieta baterii", options={"color": "#d7b928", "alpha": 0.15})
show_object(lid, name="04 pokrywa podniesiona", options={"color": "#2f6fbd", "alpha": 0.45})
