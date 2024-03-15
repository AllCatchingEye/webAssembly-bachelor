static void init_sockaddr_inet(struct sockaddr_in *addr);
int send_values(int temperature, int humidity);
void process_sensor_values(int temperature, int humidity, int status);
void check_status(int status);
void process_temperature(int temperature);
void process_humidity(int humidity);
