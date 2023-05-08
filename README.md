# PortableDesktop

**Mount portable directory as consistent user directory.**

## PortableDesktopCli

**help**
```cmd
PortableDesktopCli [options] <Target Path> <Link Path> [Home name]
        options:
                -f|--force: If the <Link Path> already exists, delete it and recreate it.
        args:
                Target Path: Specifies the physical drive and path that you want to assign to a virtual drive.
                Link Path: Specifies the virtual drive and path to which you want to assign a path.
                Home name: The subdirectory name of the <Link Path> directory, Will be specified as the value of %HOME%, which defaults to [.home].
```

**example**
```cmd
PortableDesktopCli [options] <Target Path> <Link Path> [Home name]
        options:
                -f|--force: If the <Link Path> already exists, delete it and recreate it.
        args:
                Target Path: Specifies the physical drive and path that you want to assign to a virtual drive.
                Link Path: Specifies the virtual drive and path to which you want to assign a path.
                Home name: The subdirectory name of the <Link Path> directory, Will be specified as the value of %HOME%, which defaults to [.home].

Target Path:
E:\test
Link Path:
X:\work
X:\work <<===>> E:\test
%HOME% => X:\work\.home
Microsoft Windows [10.0.19042.2846]
(c) Microsoft Corporation。

X:\work>set
HOME=X:\work\.home
HOMEDRIVE=X:
HOMEPATH=X:\work\.home
LOCALAPPDATA=X:\work\.home\AppData\Local
PORTABLE_DESKTOP_ENV_LINK_PATH=X:\work
PORTABLE_DESKTOP_ENV_TARGET_PATH=E:\test
TEMP=X:\work\.home\AppData\Local\Temp
TMP=X:\work\.home\AppData\Local\Temp
USERPROFILE=X:\work\.home
...

X:\work>
```


## PortableDesktop

**help**
```cmd
PortableDesktop [Target Path] [Link Path] [Home name]
        args:
                Target Path: Specifies the physical drive and path that you want to assign to a virtual drive.
                Link Path: Specifies the virtual drive and path to which you want to assign a path.
                Home name: The subdirectory name of the <Link Path> directory, Will be specified as the value of %HOME%, which defaults to [.home].
```

**example**

* No parameters, double click to open  
![1](./images/1.png)  

* Input parameters  
![2](./images/2.png)  

* Enter to enter the main window  
![3](./images/3.png)  

* Enter the command and press Enter to execute it. The command running environment is based on:
```cmd
HOME=X:\work\.home
HOMEDRIVE=X:
HOMEPATH=X:\work\.home
LOCALAPPDATA=X:\work\.home\AppData\Local
PORTABLE_DESKTOP_ENV_LINK_PATH=X:\work
PORTABLE_DESKTOP_ENV_TARGET_PATH=E:\test
TEMP=X:\work\.home\AppData\Local\Temp
TMP=X:\work\.home\AppData\Local\Temp
USERPROFILE=X:\work\.home
...
```

### .profile  
`%HOME%/[.profile.cmd|.profile.bat]`  
> **One of these two files is automatically executed when the program starts (if the file exists)**  

### .pd.json  
`%HOME%/.pd.json`  
> **Configuration file, currently supports shortcuts configuration**  
***Works only on PortableDesktop, not on PortableDesktopCli***

**example**
```json
{
    "shortcuts": {
        "open-url": "msedge",
        "open-file": "explorer",
        "open-home": "explorer %HOME%"
    }
}
```
* entering [open-home], [explorer %HOME%] will be executed
* entering [open-url github.com], [msedge github.com] will be executed

