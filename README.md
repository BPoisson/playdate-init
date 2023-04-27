# Playdate Windows Visual Studio CMake Project Initializer

The goal of this tool is to automate the manual processes for creating and initializing a new Playdate console project.

More details can be found in the [official Playdate C documentation](https://sdk.play.date/1.13.5/Inside%20Playdate%20with%20C.html).

### Prerequisites
1. Download and install the [Playdate SDK](https://play.date/dev/).
2. Set the `PLAYDATE_SDK_PATH` environment variable based on your SDK install location.
3. Add `<path to sdk>/bin` to your Windows PATH environment variable.
3. Install Visual Studio with C tools.
4. Install the [GNU Arm Embedded Toolchain compiler](https://developer.arm.com/Tools%20and%20Software/GNU%20Toolchain). When prompted, add to your Windows PATH environment variable. 
5. Install [CMake for Windows](https://cmake.org). When prompted, add to your Windows PATH environment variable.

### Visual Studio Set-Up for the Playdate Simulator
After creating and initializing the project, the following steps will help you to set up Visual Studio for running and debugging your project using the Playdate Simulator included in the Playdate SDK:
1. Use Visual Studio to open the `<ProjectName>.sln` file found in your new project's `build` directory.
2. Once your project is open in Visual Studio, click the drop-down button next to `Local Windows Debugger` and click `ALL_BUILD Debug Properties`.
3. Navigate to the `Debugging` section.
4. Under `Command`, clear the existing text and paste in `$(PLAYDATE_SDK_PATH)\bin\PlaydateSimulator.exe`. Then click `Ok` to close the window.
5. On the left-hand panel called `Solution Explorer`, right-click on your project's name and click `Set as Startup Project`.
6. You should now be able to run and debug your project in the Playate simulator by clicking `Local Windows Debugger` (to run with the debugger) or the empty green arrow next to it (to run without the debugger).
   1. Breakpoints should now work when set and running your project (with debugger) in the Playdate simulator.

### Current Limitations
1. This tool currently only supports C project initialization for Windows Visual Studio with CMake.
   1. The CMake step can be skipped if you would like to only generate initial C project directories and files.
   2. This tool does not currently support Lua project initialization.
2. If any of the prerequisites are skipped, this tool may not be able to complete the initialization step with CMake.
3. Only alphanumeric characters are supported in project names.
4. Only alphanumeric characters and slashes are supported in working directory paths.

### References
* https://sdk.play.date/1.13.5/Inside%20Playdate%20with%20C.html
* https://devforum.play.date/t/helpful-tip-visual-studio-debugging/3691