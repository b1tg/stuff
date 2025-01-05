
for /f "usebackq tokens=*" %%a in (`call "%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe" -latest -prerelease -products * -requires Microsoft.Component.MSBuild -property installationPath`) do (
   set "VSINSTALLPATH=%%a"
)

if not defined VSINSTALLPATH (
   echo No Visual Studio installation detected.
   goto end
)
call "%VSINSTALLPATH%\VC\Auxiliary\Build\vcvarsall.bat" amd64
cl /std:c++latest killer.c  /I"..\include"   /I"..\phlib\include" /I"..\phnt\include"  /link  /FORCE:MULTIPLE /LIBPATH:"..\phlib\bin\Release64"  phlib.lib ntdll.lib user32.lib gdi32.lib Advapi32.lib ole32.lib comdlg32.lib windowsapp.lib  winsta.lib uuid.lib windowscodecs.lib



:end
echo "end"
