param([switch]$Compress,
      [switch]$Run)

$wd = Get-Location
$build = "$wd\build"
$compressedPackage = "$build\wcgen-winx32-x64.zip"

if(Test-Path -Path $compressedPackage){
    "Compressed package exists, removing.."
    Remove-Item -Path $compressedPackage
}

"Compiling generator"
cd generator
cargo build --release
cd ..

"Packaging Application.."
electron-packager frontend --out=$build --arch=x64 --platform=win32 --app-copyright="(c) immermitternacht" --overwrite
$builtApp = "$build\wcgen-win32-x64"

"Copying artifact.."
$artifact = "$wd\generator\target\release\wcgrs.exe"
$target = "$builtApp\resources\app\rcs\wcgrs.exe"
Copy-Item -Path $artifact -Destination $target

if($Compress) {
    "Compressing.."
    Add-Type -A System.IO.Compression.FileSystem
    [IO.Compression.ZipFile]::CreateFromDirectory($builtApp, $compressedPackage)    
}

"Done!!"

if($Run) {
    "Running Application"
    cd $builtApp
    .\wcgen.exe
    cd $wd
}