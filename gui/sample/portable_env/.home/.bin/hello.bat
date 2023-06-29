@ECHO OFF

powershell -command "Add-Type -AssemblyName PresentationFramework;[System.Windows.MessageBox]::Show('Hello %1', 'Hello', 'OK', 'None')"