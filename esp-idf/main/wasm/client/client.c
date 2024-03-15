#include "curl/curl.h"
#include "curl/easy.h"
#include <stdio.h>

int main(void) {
  CURL *curl;
  CURLcode res;

  curl_global_init(CURL_GLOBAL_DEFAULT);

  curl = curl_easy_init();

  if (curl) {
    curl_easy_setopt(curl, CURLOPT_URL, "https://localhost:8000/hello_world");

    curl_easy_setopt(curl, CURLOPT_CA_CACHE_TIMEOUT, 604800L);

    res = curl_easy_perform(curl);

    if (res != CURLE_OK) {
      fprintf(stderr, "curl_easy_perform() failed %s\n",
              curl_easy_strerror(res));
    }

    curl_easy_cleanup(curl);
  }

  curl_global_cleanup();

  return 0;
}
