from pathlib import Path
import site
import sys


VENV = Path("/Users/michal/Documents/Codex/2026-07-01/mam/.venv-cq-editor")
site.addsitedir(str(VENV / "lib/python3.12/site-packages"))

SCRIPT = Path(__file__).resolve().with_name("esp32_c3_devkit_rust_case_cq_editor.py")

from PyQt5.QtCore import QTimer  # noqa: E402
from PyQt5.QtWidgets import QApplication  # noqa: E402


app = QApplication(sys.argv, applicationName="CQ-editor")

from cq_editor.main_window import MainWindow  # noqa: E402


window = MainWindow(filename=str(SCRIPT))
window.show()


def render_model() -> None:
    window.components["debugger"].render()
    window.raise_()
    window.activateWindow()


QTimer.singleShot(1200, render_model)

sys.exit(app.exec_())
