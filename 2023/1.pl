#!/usr/bin/perl

%digits = ( "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9 );
sub Digit {
    if ($_[0] =~ /[0-9]/) {
        return int($&);
    } else {
        return int(%digits{$_[0]});
    }
}

$anydigit = '(' . join('|', keys %digits) . '|[0-9])';
$total = 0;
while (<STDIN>) {
    /$anydigit/;
    $total += Digit($1)*10;
    /.*$anydigit/;
    $total += Digit($1);
}
print $total, "\n";
