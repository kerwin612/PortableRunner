@echo off

call %HOME%\.env.bat

set "PATH=%HOME%\.bin;%PATH%"

call %HOME%\.profile.java.bat
