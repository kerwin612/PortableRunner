@echo off

where java >nul 2>&1 || (echo java command not found & pause & exit 1)

set "jd_gui_url=https://github.com/java-decompiler/jd-gui/releases/download/v1.6.6/jd-gui-1.6.6.jar"
set "jd_gui_path=%HOME%\.jd-gui\jd-gui.jar"
if not exist "%jd_gui_path%" (
    mkdir %jd_gui_path%\..\ 2>nul
    echo downloading [%jd_gui_url%] to [%jd_gui_path%]
    powershell -c "Invoke-WebRequest -Uri '%jd_gui_url%' -OutFile '%jd_gui_path%'"
)
java -jar %jd_gui_path% & exit 0
