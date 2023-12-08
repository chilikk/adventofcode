use strict;
use vars qw/$line @rl %mapl %mapr $steps1 $steps2 $curnode @rl1
         @start @curnodes @path %seen $head $period $tail $steps @periods/;

sub Go {
    my ($curnode, $curdir) = @_;
    if ($curdir eq 'R') {
        return $mapr{$curnode};
    } elsif  ($curdir eq 'L') {
        return $mapl{$curnode};
    } else {
        die
    }
}

sub Lcm {
    my @nums = @_;
    my $max = @nums[0];
    for my $num (@nums) {
        $max = $num if ($num > $max);
    }
    my @primes = ();
    my $i;
    for ($i = 2; $i <= sqrt($max); $i++) {
        my $isprime = 1;
        for my $prime (@primes) {
            if ($i % $prime == 0) {
                $isprime = 0;
                last;
            }
        }
        push(@primes, $i) if ($isprime);
    }
    my @lcm = ();
    for my $num (@nums) {
        my @factors = ();
        for my $prime (@primes) {
            while ($num % $prime == 0) {
                push(@factors, $prime);
                $num /= $prime;
            }
            last if $num == 1;
        }
        push(@factors, $num) if $num != 1;
        for my $factor (@lcm) {
            for ($i = 0; $i < @factors && $factors[$i] != $factor; $i++) {}
            if ($factors[$i] == $factor) {
                splice(@factors, $i, 1);
            }
        }
        push(@lcm, @factors);
    }
    my $lcm = 1;
    for my $factor (@lcm) {
        $lcm *= $factor;
    }
    return $lcm;
}

$line = <STDIN>;
chomp $line;
@rl = split('', $line);
$line = <STDIN>;
chomp $line;
die if $line ne '';
while (<STDIN> =~ /^([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)$/) {
    $mapl{$1} = $2;
    $mapr{$1} = $3;
    if ('A' eq substr($1, 2)) {
        push @start, $1;
    }
}
$steps1 = 0;
$curnode = 'AAA';
while ($curnode ne 'ZZZ') {
    @rl1 = @rl if (!@rl1);
    $curnode = Go($curnode, shift @rl1);
    $steps1++;
}
$steps2 = 0;
@curnodes = @start;
for $curnode (@curnodes) {
    @path = ();
    %seen = ();
    @rl1 = ();
    $steps = 0;
    while (!exists($seen{$curnode}{$steps % @rl})) {
        @rl1 = @rl if (!@rl1);
        $seen{$curnode}{$steps % @rl} = 1;
        push(@path, $curnode);
        $curnode = Go($curnode, shift @rl1);
        $steps++;
    }
    $head = 0;
    while ($curnode ne @path[0]) {
        shift @path;
        $head++;
    }
    $period = @path;
    push @periods, $period;
    $tail = 0;
    while ('Z' ne substr(shift @path, 2)) {
        $tail++;
    }
    die if ($head + $tail != $period);
}
$steps2 = Lcm(@periods);
print "$steps1\n$steps2\n";
