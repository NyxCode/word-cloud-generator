# WordCloudGen

A fancy word cloud generator  

![screenshot](https://i.imgur.com/jEqSKFx.png)

## Try it out
For windows, a compiled package can be found [here](https://mega.nz/#!mepj0aQR!p8S-3gqsg9S1v_nLiT3h4M17QCXoqI7ReW51yZTdmks). For other platforms, the project has to be built manually *(see below)*

## Build on Windows
If you want to try it out, `build-windows.ps1` does all necessarry setup *(downloading & compiling the backend, packaging the frontend, ...)*. It can be run from the Windows Powershell using

-   `.\build-windows.ps1 -compress`  
    *(creates a `.zip` in `THIS_REPO\build\`)*
-   `.\build-windows.ps1 -run`
-   *(builds & starts the frontend)*

## Other Platforms
If you're not on windows, you'll have to go through the necessary steps by yourself:  
1. Clone this repository and create a package using `electron-packager CLONED_REPOSITORY` *(install it using `npm install -g electron-packager`)*. This packages the frontend into a new directory `wcgen-YOUR_PLATFORM-YOUR_ARCH`
2. Clone the [backend](https://github.com/NyxCode/wordcloudgen-backend) and compile it using `./gradlew jar`
3. Copy the compiled backend from `build/libs/` to `wcgen-YOUR_PLATFORM-YOUR_ARCH/resources/app/rcs`
