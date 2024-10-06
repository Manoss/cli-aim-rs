# cli-aim-rs
A CLI example for test de aimapi-rs lib

# build
Windows 7 and older   
```
cargo build --target i586-pc-windows-msvc
```
Windows 10 and earler
```
cargo build --target i686-pc-windows-msvc
```

# usage
From CMD command on a Foxboro station    
```
aim-rs.exe
```

#### Help    
```
aim-rs.exe -h
```

#### Examples    
Read a String
```
aim-rs.exe "CMP:BLOCK.SN0001"
```
