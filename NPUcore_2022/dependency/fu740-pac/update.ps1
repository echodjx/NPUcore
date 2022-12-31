if ($null -eq (Get-Command "svd" -errorAction SilentlyContinue)) {
    Write-Host "Update script failed to detect svdutils in the environment." 
    Write-Host "You may install it by: "
    Write-Host "    pip3 install svdtools"
    exit;
}
if ($null -eq (Get-Command "svd2rust" -errorAction SilentlyContinue)) {
    Write-Host "Update script failed to detect svd2rust in the environment." 
    Write-Host "You may install it by: "
    Write-Host "    cargo install svd2rust"
    exit;
}
if ($null -eq (Get-Command "form" -errorAction SilentlyContinue)) {
    Write-Host "Update script failed to detect form in the environment." 
    Write-Host "You may install it by: "
    Write-Host "    cargo install form"
    exit;
}
svd patch fu740.yaml
svd2rust --target riscv -i fu740.svd.patched
Remove-Item -Path "src" -Recurse -errorAction SilentlyContinue
form -i lib.rs -o src/ 
Remove-Item lib.rs
cargo fmt
