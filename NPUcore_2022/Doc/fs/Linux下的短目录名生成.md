
# Table of Contents

1.  [代码注释](#orgce7147d)
    1.  [其函数的头部声明](#orgef4c813)
    2.  [扩展名部分识别](#org63a3f3f)
    3.  [基本名部分识别](#org2de5e40)
    4.  [基本名生成](#org7aa5328)
    5.  [扩展名生成](#org303e65b)
    6.  [文件名拷贝和特殊情况判断](#org82bc24b)
    7.  [唯一扩展数字生成](#org123736b)


<a id="orgce7147d"></a>

# 代码注释


<a id="orgef4c813"></a>

## 其函数的头部声明

    static int vfat_create_shortname(
    /*                             */struct inode *dir, 
                                     struct nls_table *nls,
                                     wchar_t *uname,
                                     int ulen,
                                         unsigned char *name_res, 
                                     unsigned char *lcase)
    {
            wchar_t *ip, // 迭代器指针,指向原文件名uname
                *ext_start, // 扩展名开始位置的指针,指向'.'
                *end, // 长文件名结尾位置的指针
                *name_start; // 指向文件名的首个字符
            unsigned char base[9], // 基本名
                      ext[4], // 扩展名
                      buf[8], 
                      *p; // 迭代器指针,指向base,ext或者buf之类的点
    
    // 用于宽字符向UTF-8转换接收结果的字符串缓存
            unsigned char charbuf[NLS_MAX_CHARSET_SIZE];
        // 接下来是用于unicode转utf-8储存在u8的两个变量
            int chl, // 一个宽字符需要的短字符的长度
            chi; // 可以认为是指针/迭代器的iter变量
    
            int sz = 0, // 需要转换为的基本名的部分的长度
            extlen, // 扩展名长度
            baselen, // 基本名长度
            i,
            numtail_baselen,
            numtail2_baselen;
            int is_shortname;
            struct shortname_info 
            base_info, 
            ext_info;
            unsigned short opt_shortname = 
                       MSDOS_SB(dir->i_sb)->options.shortname;
    
        // 默认为: 不需要长文件名(是符合"8.3"标准的短文件名)
            is_shortname = 1;
    
        // 记录短文件名更改的信息
            INIT_SHORTNAME_INFO(&base_info);
            INIT_SHORTNAME_INFO(&ext_info);


<a id="org63a3f3f"></a>

## 扩展名部分识别

从尾部向头部寻找扩展名,从而给出扩展名是否存在和如果存在,其主文件名长度(也就基本知道其扩展名的长度)如何
检测方式为:从最后一位开始循环到第一个字符,如果指针指向点,则准备退出循环,对点的位置在最后一位的,则认为其没有扩展名,主文件名长度为此文件名长度

    ext_start = end = &uname[ulen];
    while (--ext_start >= uname) {
            if (*ext_start == 0x002E) {	/* is `.' */
                    if (ext_start == end - 1) {
                            sz = ulen;
                            ext_start = NULL;
                    }
                    break;
            }
    }

根据其结果向量(ext<sub>start,sz</sub>)的第一位ext<sub>start判断结果</sub>
如果其扩展名的指针指向名字外(首个字符之前),则之前的while是跑完退出,则其对应的结果也是没有扩展名的
否则,如果扩展名开始指针没有被设置为空,则是有扩展名的情况.


<a id="org2de5e40"></a>

## 基本名部分识别

对有扩展名的情况,

    if (ext_start == uname - 1) {
            sz = ulen;
            ext_start = NULL;
    } else if (ext_start) {
            /*
             * Names which start with a dot could be just
             * an extension eg. "...test".  In this case Win95
             * uses the extension as the name and sets no extension.
             */
            name_start = &uname[0];
      // 跳过vfat标准要求跳过的字符(空格和点)
            while (name_start < ext_start) {
                    if (!vfat_skip_char(*name_start))
                            break;
                    name_start++;
            }
      // 跳过之后
            if (name_start != ext_start) {
                    sz = ext_start - uname; //
                    ext_start++; 
            } else {
                    sz = ulen;// 扩展名长度设为
                    ext_start = NULL;//也就是Linus注释提到的情况
              // 直接将扩展名定为基本名
            }
    }

注意C语言注释提到,Win95将纯扩展名形式的文件名的扩展名的作为文件名,而完全不设"扩展名"
文件名最后一个字符为'.'和文件全局没有出现过一次'.',不存在扩展名


<a id="org7aa5328"></a>

## 基本名生成

从最开始字符替换所有的非法字符直至基本名结束

     numtail_baselen = 6;
     numtail2_baselen = 2;
     for (baselen = i = 0, p = base, ip = uname; 
    i < sz;
    i++, ip++) 

以下是UTF32到UTF8的转换相关代码

    {         // 转换编码同时跳过非法字符
                    chl = to_shortname_char(nls, 
                                      charbuf, 
                                      sizeof(charbuf),
                                            ip, &base_info);
                    if (chl == 0) // 当前宽字符为被跳过字符,无法转化为
                            // 合法u8,则下一个
                            continue;
              // 似乎是为了保证在替换数字结尾时其中不会出现残存字符(半个UTF8)
                    if (baselen < 2 && (baselen + chl) > 2)
                            numtail2_baselen = baselen;
                    if (baselen < 6 && (baselen + chl) > 6)
                            numtail_baselen = baselen;

将替换好的基本名复制到基本名的数组中(p已经提前初始化为base的地址),同时增长基本名长度
基本名长度等于8,判断:如果此时长度不足以表达整个编码,则此时必须使用长文件名,否则短文件名其实也够,此外其应当结束循环退出
此外,一个全部由需要跳过字符组成的字符串无法作为合法的FAT文件名

            for (chi = 0; chi < chl; chi++) {
                    *p++ = charbuf[chi];
                    baselen++;
                    if (baselen >= 8)
                            break;
            }
            if (baselen >= 8) {
                    if ((chi < chl - 1) || (ip + 1) - uname < sz)
                            is_shortname = 0;
                    break;
            }
    }
    if (baselen == 0) {
            return -EINVAL;
    }


<a id="org303e65b"></a>

## 扩展名生成

    extlen = 0;
            if (ext_start) {// 如果存在扩展名
                    for (p = ext, ip = ext_start; 
                   extlen < 3 && ip < end; ip++) {
                            chl = to_shortname_char(
                                              nls, 
                                              charbuf,
                                              sizeof(charbuf),
                                                    ip,
                                              &ext_info);
                            if (chl == 0) // 当前宽字符为被跳过字符,无法转化为
                                    // 合法u8,则下一个
                                    continue;
    
                            if ((extlen + chl) > 3) {
                                    is_shortname = 0;
                                    break;
                            }//超过3个字符,直接退出且需要使用长文件名
    
                            for (chi = 0; chi < chl; chi++) {
                                    *p++ = charbuf[chi];
                                    extlen++;
                            }//将其复制到ext数组
    
                            if (extlen >= 3) {
                                    if (ip + 1 != end)
                                            is_shortname = 0;
                                    break;
                            }//如果扩展名超过3个字符还没结束,直接退出且需要使用长文件名
                    }
            }
            ext[extlen] = '\0'; // 写入'\0'为一会儿复制制造"墙"
            base[baselen] = '\0';
    
    // 防止偶尔发生的将UNUSED标记写入短文件名的情况,将其改为0x05,
    // 尽管这可能会导致字符显示错误,但比起被标记为UNUSED, 这还是好多了
            /* Yes, it can happen. ".\xe5" would do it. */
            if (base[0] == DELETED_FLAG)
                    base[0] = 0x05;


<a id="org82bc24b"></a>

## 文件名拷贝和特殊情况判断

这一段主要是判断各种的特殊情况和决定是否提前返回,为其文件系统提供帮助
同时,也决定是否生成数字后缀

            /* OK, at this point we know that base is not longer than 8 symbols,
             * ext is not longer than 3, base is nonempty, both don't contain
             * any bad symbols (lowercase transformed to uppercase).
             */
    // 按照标准,将整个11个字符的函数初始化为空格
            memset(name_res, ' ', MSDOS_NAME);
    // 复制基本名到用户提供的数组
            memcpy(name_res, base, baselen);
    // 复制扩展名到用户提供的数组
            memcpy(name_res + 8, ext, extlen);
            *lcase = 0;
    
            if (is_shortname && base_info.valid && ext_info.valid) {
                    if (vfat_find_form(dir, name_res) == 0)
                            return -EEXIST;//判重,同名则拒绝生成
    
                    if (opt_shortname & VFAT_SFN_CREATE_WIN95) {//WIN95模式
                            return (base_info.upper && ext_info.upper);
                    } else if (opt_shortname & VFAT_SFN_CREATE_WINNT) {
                      // WIN NT模式,注意,这实际上是第一个有自动生成短文件名算法的版本
                            if ((base_info.upper || base_info.lower) &&
                                (ext_info.upper || ext_info.lower)) {
                                    if (!base_info.upper && base_info.lower)
                                            *lcase |= CASE_LOWER_BASE;// 检测其是否基本名大写
                                    if (!ext_info.upper && ext_info.lower)
                                            *lcase |= CASE_LOWER_EXT;//检测扩展名大写情况
                                    return 1;
                            }
                            return 0;
                    } else {// 其他模式目前不支持,应当报告BUG
                            BUG();
                    }
            }
    // 从文件系统的元数据段获取数字结尾的选项,检查其目前是否有该数据
            if (MSDOS_SB(dir->i_sb)->options.numtail == 0)
                    if (vfat_find_form(dir, name_res) < 0)
                            return 0;//如果没发现"重名"(类似名称),可以退出了,不用生成数字


<a id="org123736b"></a>

## 唯一扩展数字生成

按照其微软提供的技术规格,这里的算法其实
注意看这段开头的注释,完美的算法可能并不存在,只是比较合适而已.

            /*
             * Try to find a unique extension.  This used to
             * iterate through all possibilities sequentially,
             * but that gave extremely bad performance.  Windows
             * only tries a few cases before using random
             * values for part of the base.
             */
    // 如果基本文件名超过6位(7或者8),则其文件名
            if (baselen > 6) {//这时候优先用最后两位
                    baselen = numtail_baselen;
                    name_res[7] = ' ';
            }
    // 将文件基本名长度处替换为~,尝试单个数字
            name_res[baselen] = '~';
            for (i = 1; i < 10; i++) {
                    name_res[baselen + 1] = i + '0';
    // 如果单个数字能确保找不到,则用单个数字
                    if (vfat_find_form(dir, name_res) < 0)
                            return 0;
            }
    //好吧,如果到这里就是都找到了
    //然后开始伪随机
            i = jiffies & 0xffff;
            sz = (jiffies >> 16) & 0x7;
            if (baselen > 2) {//如果基本名长度超过2则使用6位数字
                    baselen = numtail2_baselen;
                    name_res[7] = ' ';
            }
    
            name_res[baselen + 4] = '~';
            name_res[baselen + 5] = '1' + sz;
            while (1) {
                    sprintf(buf, "%04X", i);
                    memcpy(&name_res[baselen], buf, 4);
                    if (vfat_find_form(dir, name_res) < 0)
                            break;
                    i -= 11;
            }

