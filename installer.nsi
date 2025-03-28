; NSIS
!include "MUI2.nsh"

Name "Toolboxer"
OutFile "toolboxer_installer.exe"
InstallDir "$PROGRAMFILES\Toolboxer"

!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

Section "MainSection" SEC01
    SetOutPath "$INSTDIR"
    File /r "target\release\toolboxer.exe"
    
    ReadRegStr $0 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path"
    StrCpy $1 "$0;$INSTDIR"
    WriteRegStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path" "$1"
    System::Call 'user32::SendMessageTimeout(i 0xFFFF, i 0x1A, i 0, w "Environment", i 2, i 5000, i 0)'
    
    CreateShortCut "$SMPROGRAMS\Toolboxer.lnk" "$INSTDIR\toolboxer.exe"
SectionEnd

!insertmacro MUI_LANGUAGE "SimpChinese"