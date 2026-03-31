// ovenTime returns the amount in minutes that the lasagna should stay in the
// oven.
int ovenTime() {
    return 40;
}

/* remainingOvenTime returns the remaining
   minutes based on the actual minutes already in the oven.
*/
int remainingOvenTime(int minutes_in_oven) {
    return ovenTime() - minutes_in_oven;
}

/* preparationTime returns an estimate of the preparation time based on the
   number of layers and the necessary time per layer.
*/
int preparationTime(int number_of_layers) {
    constexpr int kMinutesPerLayer {2};
    return number_of_layers * kMinutesPerLayer;
}

// elapsedTime calculates the total time spent to create and bake the lasagna so
// far.
int elapsedTime(int number_of_layers, int minutes_in_oven) {
    return preparationTime(number_of_layers) + minutes_in_oven;
}
