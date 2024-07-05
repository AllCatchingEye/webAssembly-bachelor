#define MAX_TEMPERATURE 50
#define MAX_HUMIDITY 100

#define MIN_TEMPERATURE 0
#define MIN_HUMIDITY 0

#define BUFFER_SIZE 10

#define TRUE 1
#define FALSE 0

int main();

typedef struct {
  int temperature;
  int humidity;
  int status;
} dht11_values_t;

int initilize_sensors();
dht11_values_t read_dht11_sensor();

int get_wifi_status();

int valid_values(dht11_values_t dht11_values);
int valid_status(dht11_values_t dht11_values);

/*** Exposed native APIs ***/

void start_server();

int read_temperature();
int read_humidity();
int read_status();

void build_message(char *buffer, int buffer_len, int temperature, int humidity);

/*** Message Queue ***/
void fifo_init();
int put(char *msg);
