U_FAT32_DIR="../easy-fs-fuse"
U_FAT32=$1

touch ${U_FAT32}
sudo dd if=/dev/zero of=${U_FAT32} bs=1M count=200
sudo mkfs.vfat -F 32 ${U_FAT32}
sudo fdisk -l ${U_FAT32}

if test -e ${U_FAT32_DIR}/fs
then 
    sudo rm -r ${U_FAT32_DIR}/fs
fi
sudo mkdir ${U_FAT32_DIR}/fs

sudo mount -f ${U_FAT32} ${U_FAT32_DIR}/fs
if [ $? ]
then
    sudo umount ${U_FAT32}
fi
sudo mount ${U_FAT32} ${U_FAT32_DIR}/fs

# build root
sudo mkdir -p ${U_FAT32_DIR}/fs/etc
sudo mkdir -p ${U_FAT32_DIR}/fs/bin
sudo mkdir -p ${U_FAT32_DIR}/fs/root
sudo sh -c "echo -e "root:x:0:0:root:/root:/bash\n" > ${U_FAT32_DIR}/fs/etc/passwd"
sudo touch ${U_FAT32_DIR}/fs/root/.bash_history

for programname in $(ls ../user/src/bin)
do
    sudo cp -r ../user/target/riscv64gc-unknown-none-elf/release/${programname%.rs} ${U_FAT32_DIR}/fs/${programname%.rs}
done

sudo cp -r ../bash-5.1.16/bash ${U_FAT32_DIR}/fs/bin/

sudo umount ${U_FAT32_DIR}/fs
