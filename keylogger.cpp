#include <iostream>
#include <fcntl.h>
#include <unistd.h>
#include <linux/input.h>
#include <sys/stat.h>
#include <fstream>

#define LOGFILE "/tmp/data"
#define LOG "./log"


int main(int argc, char** argv){
    struct input_event ev;
    int fd = open("/dev/input/event23", O_RDONLY); //Replace with your keyinput log file (eg. event3) 
    FILE* fp = fopen(LOGFILE, "a");
    
    std::ofstream lf;
    lf.open(LOG, std::ios_base::app);

    std::cout << "press Esc to quit\n";
    while(true){
        read(fd, &ev, sizeof(ev));
        if((ev.type == EV_KEY) && (ev.value == 0)){
            std::cout << ev.code << std::endl;
            lf << ev.code << std::endl;
            if(ev.code == 1){
                break;
            }
        }
    }
    lf.close();
}