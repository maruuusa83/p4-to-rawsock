// Copyright (c) 2018 Daichi Teruya @maruuusa83
// This project is released under the MIT license
// https://github.com/maruuusa83/raw_sock_rust/blob/master/LISENCE
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include <sys/socket.h>
#include <arpa/inet.h>
#include <linux/if_packet.h>
#include <net/ethernet.h>
#include <net/if.h>
#include <sys/ioctl.h>

int ioctl_siocgifindex(int fp, struct ifreq *ifr)
{
    return ioctl(fp, SIOCGIFINDEX, ifr);
}

int ioctl_siocgifhwaddr(int fp, struct ifreq *ifr)
{
    return ioctl(fp, SIOCGIFHWADDR, ifr);
}

void ioctl_set_promisc(int fp, struct ifreq *ifr)
{
    ioctl(fp, SIOCGIFFLAGS, &ifr);
    ifr->ifr_flags |= IFF_PROMISC;
    ioctl(fp, SIOCSIFFLAGS, &ifr);
}

void flush_recv_buf(int pd)
{
    unsigned char buf[2048];

    int i;
    do {
        fd_set fds;
        struct timeval t;
        FD_ZERO(&fds);
        FD_SET(pd, &fds);
        memset(&t, 0, sizeof(t));
        i = select(FD_SETSIZE, &fds, NULL, NULL, &t);
        if (i > 0) recv(pd, buf, i, 0);
    } while (i);
}
