# FTP协议介绍
FTP（File Transfer Protocol，文件传输协议）是 TCP/IP 协议组中的协议之一。其主要作用是在服务器和客户端之间实现文件的传输和共享。FTP协议运行在TCP连接上，保证了文件传输的可靠性。

FTP使用了两个并行的tcp来传输文件：一个是控制连接，使用21号端口；一个是数据连接，使用20号端口，控制连接用于在两个主机之间传输控制信息，如口令，用户标识，存放、获取文件等命令。数据连接用于实际发送一个文件，发送完文件之后数据连接会关闭。

FTP主要有主动模式和被动模式两种。主动模式是FTP客户端告诉服务端用哪个端口作为数据端口，然后让服务端来连接自己。被动模式则是由客户端发起控制连接请求和数据连接请求。

# ubuntu下搭建FTP服务器
本节搭建一个FTP服务器，可用于我们后续进行测试。

* 创建文件夹
```
cd ~
mkdir ftpserver
```
* 安装ftp服务器
```
sudo apt-get install vsftpd
```
* 配置vsftpd.conf文件：
```
sudo vim /etc/vsftpd.conf
```
添加如下：
```
anonymous_enable=NO
anon_root=/home/xxx/ftpserver
no_anon_password=YES
write_enable=YES
anon_upload_enable=YES
anon_mkdir_write_enable=YES
```
* 添加组用户
```
sudo groupadd ftpgroup
```
* 增加用户，例如用户名为tt
```
sudo useradd -g ftpgroup -d ~/ftpserver/tt -M tt
```
* 创建tt对应的文件夹tt
```
mkdir tt //在ftpserver目录下
sudo chmod 777 tt
```
* 设置密码
```
sudo passwd tt
```
* 重启vsftpd
```
sudo /etc/init.d/vsftpd restart
```

# 测试
* 打开终端，输入：
```
ftp 127.0.0.1
```
* 输入用户名tt
* 输入密码
* 使用get命令下载
* 使用put命令上传