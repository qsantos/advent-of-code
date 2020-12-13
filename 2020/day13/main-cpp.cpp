#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <limits>
#include <vector>
#include <tuple>

using std::cout;
using std::endl;
using std::min_element;
using std::numeric_limits;
using std::streamsize;
using std::string;
using std::vector;
using std::transform;
using std::tuple;

using Bus = tuple<int64_t, int64_t>;
using Buses = vector<Bus>;

int64_t extended_gcd(int64_t a, int64_t b, int64_t* x, int64_t* y) {
    int64_t old_r = a, r = b;
    int64_t old_s = 1, s = 0;
    int64_t old_t = 0, t = 1;
    while (r) {
        int64_t q = old_r / r;
        int64_t tmp_r = r; r = old_r - q * r; old_r = tmp_r;
        if (x) { int64_t tmp_s = s; s = old_s - q * s; old_s = tmp_s; }
        if (y) { int64_t tmp_t = t; t = old_t - q * t; old_t = tmp_t; }
    }
    if (x) { *x = old_s; };
    if (y) { *y = old_t; };
    return old_r;
}


int64_t invert(int64_t a, int64_t m) {
    int64_t x;
    int64_t g = extended_gcd(a, m, &x, NULL);
    if (g != 1) {
        printf("Inverse doesn't exist");
        exit(1);
    }
    return (x % m + m) % m;
}

int64_t crt(const vector<int64_t>& remainders, const vector<int64_t>& modulos) {
    assert(remainders.size() == modulos.size());
    int64_t N = 1;
    for (const auto& modulo: modulos) {
        N *= modulo;
    }
    int64_t x = 0;
    for (size_t i = 0; i < remainders.size(); i++) {
        const auto ai = remainders[i];
        const auto ni = modulos[i];
        const auto Ni = N / ni;
        const auto Mi = invert(Ni, ni);
        x += ai * Ni * Mi;
        x %= N;
    }
    return x;
}

void puzzle1(const Buses& buses, int64_t earliest) {
    auto [_, bus_id] = *min_element(
        buses.begin(),
        buses.end(),
        [earliest](const Bus& lbus, const Bus& rbus) {
            const auto [loffset, lbus_id] = lbus;
            const auto [roffset, rbus_id] = rbus;
            const auto lwait = lbus_id - (earliest % lbus_id);
            const auto rwait = rbus_id - (earliest % rbus_id);
            return lwait < rwait;
        }
    );
    auto wait = bus_id - (earliest % bus_id);
    cout << bus_id * wait << endl;
}

void puzzle2(const Buses& buses) {
    vector<int64_t> remainders(buses.size());
    transform(buses.begin(), buses.end(), remainders.begin(), [](const Bus& bus) {
        const auto [offset, bus_id] = bus;
        return bus_id - (offset % bus_id);
    });
    vector<int64_t> modulos(buses.size());
    transform(buses.begin(), buses.end(), modulos.begin(), [](const Bus& bus) {
        const auto [offset, bus_id] = bus;
        return bus_id;
    });
    cout << crt(remainders, modulos) << endl;
}

int main(void) {
    std::ifstream f("input");
    if (!f.is_open()) {
        cout << "Could not open file" << endl;
        exit(1);
    }

    int64_t earliest;
    f >> earliest;
    f.ignore(numeric_limits<streamsize>::max(), '\n');

    Buses buses;
    string bus_id;
    int64_t offset = 0;
    while (getline(f, bus_id, ',')) {
        if (bus_id.back() == '\n') {
            bus_id = bus_id.substr(0, bus_id.length() - 1);
        }
        if (bus_id != "x") {
            buses.push_back({offset, stoul(bus_id)});
        }
        offset++;
    }

    puzzle1(buses, earliest);
    puzzle2(buses);
}
