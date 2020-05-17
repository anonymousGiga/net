## TFTP介绍

TFTP（Trivial File Transfer Protocol,简单文件传输协议）是TCP/IP协议族中的一个用来在客户机与服务器之间进行简单文件传输的协议，提供不复杂、开销不大的文件传输服务。TFTP基于UDP，对应端口号为69.

## TFTP报文格式
### TFTP协议

```

   | 2bytes | String   | 2bytes | String | 2bytes |

   ------------------------------------------------

   | Opcode | Filename |    0   |  Mode  |   0    |

   ------------------------------------------------

```

* Opcode字段

   操作码/命令

```

   --------------------------------------------------

   | Opcode | Command      | Description            |

   --------------------------------------------------

   |  1     | Read Request | Request to read a file |

   |  2     | Write Request| Request to write a file|

   |  3     | File Data    | Transfer of file data  |

   |  4     | Data Ack     | Ack of file data       |

   |  5     | Error        | Error indication       |

   --------------------------------------------------

```

* Filename字段

   文件的名字。

* Mode字段

   数据模式。协议传输的文件数据格式，可以是NetASCII，也可以是标准ASCII，八位二进制数据或邮件标准ASCII。在邮件模式已经不再支持。

### 报文

* 读写报文

```

   | 2bytes     | String   | 2bytes | String | 2bytes |

   ------------------------------------------------

   | Opcode=1/2 | Filename |    0   |  Mode  |   0    |

   ------------------------------------------------

```

* 数据报文

```

   | 2bytes   | 2bytes       | 512 bytes|

   --------------------------------------

   | Opcode=3 | block number | Data     |

   --------------------------------------

```

* ACK报文

```

   | 2bytes   | 2bytes       |

   ---------------------------

   | Opcode=4 | block number |

   ---------------------------

```

* ERROR报文

```

   | 2bytes   | 2bytes       | N bytes       |

   -------------------------------------------

   | Opcode=5 | error number | error message |

   -------------------------------------------

```

## 传输过程

```

1、客户端发送读/写请求；

2、服务端同意请求，文件以固定的512字节块的长度发送；

3、再发送下一个包之前，需要先等待之前的块得到确认;

4、如果包中数据少于512字节，则说明是最后一个数据包，传输结束;

5、如果包丢失，则超时重传。

```







