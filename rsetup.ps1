function Usage {
    Write-Host "Usage: $PSCommandPath [daynumber]"
}

if ( $args.Count -ne 1 ) {
    Write-Host "Incorrect number of arguments"
    Usage
    Exit 1
} 

if ( (-Not ($args[0] -match "^[0-9]+$")) -or ( $args[0] -le 0 ) ) {
    Write-Host "Argument not a positive integer"
    Usage
    Exit 1
}

$daynumber=$args[0]
$filename=".\src\day{0:D2}.rs" -f $daynumber
$cargo_str="`n[[bin]]`nname = `"day{0:D2}`"`npath = `"src/day{0:D2}.rs`"" -f $daynumber

if ( Test-Path -Path $filename -PathType Leaf ) {
    Write-Host "File `"$filename`" already exists"
    Exit 1
}

Add-Content .\Cargo.toml $cargo_str

$replace_str="let day: u32 = {0}" -f $daynumber
((Get-Content -Path .\template.rs -Raw) -replace "let day: u32 = XX", $replace_str) | Set-Content -Path $filename

