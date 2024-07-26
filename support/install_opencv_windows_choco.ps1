$CHOCO_LLVM_VERSION=18.1.8
$OPENCV_VERSION=4.10.0
$OPENCV_VERSION_LIB=4100

# choco install -y llvm --version %CHOCO_LLVM_VERSION%
# choco install -y opencv --version %OPENCV_VERSION%

$OPENCV_PATH = "C:/tools/opencv/build/x64/vc16/bin;C:/tools/opencv/build/x64/vc15/bin"

$NEW_PATH = [Environment]::GetEnvironmentVariable("Path", "User")
$NEW_PATH += ";$OPENCV_PATH"
[Environment]::SetEnvironmentVariable("Path", $NEW_PATH, "User")
[Environment]::SetEnvironmentVariable("OPENCV_LINK_LIBS", "opencv_world" + $OPENCV_VERSION_LIB, "User")
[Environment]::SetEnvironmentVariable("OPENCV_LINK_PATHS", "C:/tools/opencv/build/x64/vc16/lib,C:/tools/opencv/build/x64/vc15/lib", 'User')
[Environment]::SetEnvironmentVariable("OPENCV_INCLUDE_PATHS", "C:/tools/opencv/build/include", "User")

