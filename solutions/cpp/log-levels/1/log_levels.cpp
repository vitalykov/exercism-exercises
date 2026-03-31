#include <string>

namespace log_line {

std::string message(std::string line) {
    size_t delim_pos {line.find(':')};
    return line.substr(std::min(line.length(), delim_pos + 2));
}

std::string log_level(std::string line) {
    size_t start_pos {line.find('[')};
    size_t end_pos {line.find(']')};
    return line.substr(start_pos + 1, end_pos - start_pos - 1);
}

std::string reformat(std::string line) {
     return message(line) + " (" + log_level(line) + ")";
}

}  // namespace log_line
