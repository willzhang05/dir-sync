#include <iostream>
#include <fstream>
#include <streambuf>
#include <nlohmann/json.hpp>
#include <boost/asio.hpp>
#include <libssh/libsshpp.hpp>

using json = nlohmann::json;

/*class Server {
    public:
    std::string name;
    std::string hostname;
    boost::asio::ip::address addr;
    std::string remote_dir;
    int port;

    Server(json &server_config) {
    
    }
    
};*/

int read_in(json &config) {
    std::ifstream config_file("config.json");
    try {
        config = json::parse(config_file);
    } catch (json::exception &e) {
        printf("Error parsing json configuration: %s", e.what());
        return 1;
    }
    return 0;
}

int main() {
    json config;
    int read = read_in(config);
    if (read == 1) {
        return 1;
    }
    //Server test = Server(config["servers"_json_pointer]);
    std::vector<json> servers = config["servers"];
    for (int i = 0; i < (int) sizeof(servers); i++) {
        
    }
    std::cout<<servers;
    
    return 0;
}
