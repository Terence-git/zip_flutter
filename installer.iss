#define MyAppName "zipf"
#define MyAppVersion "1.1"
#define MyAppPublisher "Térence Martinant"
#define MyAppExeName "zipf.exe"

[Setup]
AppId={{A9B8C7D6-E5F4-4321-8765-43210FEDCBA9}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
; Dossier d'installation par défaut (ex: C:\Program Files\zipf)
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
; Nom du fichier installeur généré
OutputBaseFilename=zipf_setup
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64compatible
ChangesEnvironment=yes

[Files]
; Indique le chemin vers ton fichier .exe compilé en release
Source: "target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Tasks]
; Option d'ajout au PATH cochée par défaut
Name: "envPath"; Description: "Ajouter {#MyAppName} aux variables d'environnement (PATH)"; Flags: unchecked

[Code]
// Fonction Pascal Script pour ajouter le dossier d'installation au PATH système
procedure RegisterPreviousData(PreviousDataKey: Integer);
var
  Path: String;
begin
  if IsTaskSelected('envPath') then
  begin
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
    begin
      if Pos(ExpandConstant('{app}'), Path) = 0 then
      begin
        Path := Path + ';' + ExpandConstant('{app}');
        RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
      end;
    end;
  end;
end;

procedure CurUninstallStepChanged(UninstallStep: TUninstallStep);
var
  Path, AppDir: String;
  P: Integer;
begin
  // Retirer du PATH lors de la désinstallation
  if UninstallStep = usPostUninstall then
  begin
    AppDir := ExpandConstant('{app}');
    if RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
    begin
      P := Pos(';' + AppDir, Path);
      if P > 0 then
      begin
        Delete(Path, P, Length(';' + AppDir));
        RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
      end
      else
      begin
        P := Pos(AppDir + ';', Path);
        if P > 0 then
        begin
          Delete(Path, P, Length(AppDir + ';'));
          RegWriteStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
        end;
      end;
    end;
  end;
end;