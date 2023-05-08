pragma circom 2.0.0;

template AgeVerification(minimum_age) {
    // YYYYMMDD
    signal input birthdate;
    signal input current_date;
    signal output age_above_threshold;

    // Compute the age
    var age = (current_date - birthdate) / 10000;

    // Check if the age is above the threshold
    age_above_threshold <== age * (age - minimum_age + 1);
}

component main {public [current_date]} = AgeVerification(18);
