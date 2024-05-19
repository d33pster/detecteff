@echo off
:: BatchGotAdmin
:--------------------------
@Rem --> Check for Permission
>nul 2>&1 "%SYSTEMROOT%\system32\cacls.exe" "%SYSTEMROOT%\system32\config\system"

@Rem if error flag set, we do not have admin.
if %errorlevel% NEQ 0 (
    goto UACPrompt
) else ( goto gotAdmin )

:UACPrompt
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    echo UAC.ShellExecute "%~s0", "", "", "runas", 1 >> "%temp%\getadmin.vbs"

    "%temp%\getadmin.vbs"
    exit /B

:gotAdmin
    if exist "%temp%\getadmin.vbs" ( del "%temp%\getadmin.vbs" )
    pushd "%CD%"
    CD /D "%~dp0"
:-----------------------------------

@Rem check registry for ansi colour code
echo detecteff ANSI colour fix for Windows
echo version 1.1
echo copyright d33pster (Soumyo Deep Gupta)
echo.
echo INIT
echo.
echo [registry]
reg query HKCU\Console\ | find /I "VirtualTerminalLevel" > nul
if %errorlevel% NEQ 0 (
    @Rem setting doesnt exist
    reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1
    echo added registry for ANSI escape sequence.
) else (
    echo found existing registry for ANSI escape sequence. Skipping..
)

echo.

setlocal

set /p choke=

endlocal