CALL %PORTABLE_RUNNER_ENV_LINK_PATH%\HostEnv.bat

IF NOT "%SVN_HOME%" == "" (
    SET "PATH=%SVN_HOME%\bin;%PATH%"
)

IF NOT "%GIT_HOME%" == "" (
    SET "PATH=%GIT_HOME%\bin;%PATH%"
)

IF NOT "%CODE_HOME%" == "" (
    SET "PATH=%CODE_HOME%;%PATH%"
)

IF NOT "%IDEA_HOME%" == "" (
    SET "PATH=%IDEA_HOME%\bin;%PATH%"
)

IF NOT "%NODE_HOME%" == "" (
    SET "PATH=%NODE_HOME%;%PATH%"
)

IF NOT "%JAVA_HOME%" == "" (
    SET "PATH=%JAVA_HOME%\bin;%PATH%"
)

IF NOT "%MAVEN_HOME%" == "" (
    SET "PATH=%MAVEN_HOME%\bin;%PATH%"
)
