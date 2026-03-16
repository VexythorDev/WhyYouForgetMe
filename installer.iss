[Setup]
AppName=WhyYouForgetMe
AppVersion=1.0
AppPublisher=VexythorDev
DefaultDirName={autopf}\WhyYouForgetMe
DefaultGroupName=WhyYouForgetMe
OutputDir=dist
OutputBaseFilename=PlantaSetup
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "brazilianportuguese"; MessagesFile: "compiler:Languages\BrazilianPortuguese.isl"

[Tasks]
Name: "desktopicon"; Description: "Criar atalho na area de trabalho"; GroupDescription: "Atalhos:"; Flags: checkedonce

[Files]
Source: "target\x86_64-pc-windows-msvc\release\plant.exe";      DestDir: "{app}"; Flags: ignoreversion
Source: "target\x86_64-pc-windows-msvc\release\SDL2.dll";       DestDir: "{app}"; Flags: ignoreversion
Source: "target\x86_64-pc-windows-msvc\release\SDL2_image.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "target\x86_64-pc-windows-msvc\release\SDL2_ttf.dll";   DestDir: "{app}"; Flags: ignoreversion
Source: "assets\banco.md";           DestDir: "{app}\assets"; Flags: ignoreversion
Source: "assets\config.toml";        DestDir: "{app}\assets"; Flags: ignoreversion
Source: "assets\sprites\*";          DestDir: "{app}\assets\sprites"; Flags: ignoreversion recursesubdirs

[Icons]
Name: "{group}\WhyYouForgetMe";       Filename: "{app}\plant.exe"
Name: "{group}\Desinstalar";          Filename: "{uninstallexe}"
Name: "{autodesktop}\WhyYouForgetMe"; Filename: "{app}\plant.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\plant.exe"; Description: "Abrir WhyYouForgetMe agora!"; Flags: nowait postinstall skipifsilent
