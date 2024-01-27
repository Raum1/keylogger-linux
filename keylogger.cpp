#include <iostream>
#include <fcntl.h>
#include <unistd.h>
#include <linux/input.h>
#include <sys/stat.h>

#define LOGFILE "/tmp/data"
#define LOG "./log"


int main(int argc, char** argv){
    struct input_event ev;
    int fd = open("/dev/input/event23", O_RDONLY); //Replace with your keyinput log file (eg. event3) 
    FILE* fp = fopen(LOGFILE, "a");
    FILE* lf = fopen(LOG, "a");
    std::cout << "press Esc to quit\n";
    while(true){
        read(fd, &ev, sizeof(ev));
        if((ev.type == EV_KEY) && (ev.value == 0)){
            std::cout << ev.code << std::endl;
            fprintf(lf, "%d\n", ev.code);
            if(ev.code == 1){
                fclose(lf);
                break;
            }
        }
    }
}