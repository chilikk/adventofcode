use POSIX qw/ceil/;
use POSIX qw/floor/;

sub NumberOfWays ($$) {
    my ($time, $distance) = @_;
    # $time * x - x**2 = $distance
    # x**2 - $time*x + $distance = 0
    # (x - $time/2) ** 2 = $time ** 2 / 4 - $distance
    # x = +- sqrt ($time ** 2 / 4 - $distance) + $time/2
    # increase distance by 0.5 to make sure we win
    my $x1 = sqrt($time ** 2 / 4 - $distance - 0.5) + $time / 2;
    my $x2 = -sqrt($time ** 2 / 4 - $distance - 0.5) + $time / 2;
    my $n = floor($x1) - ceil($x2) + 1;
    $n = $n > 0 ? $n : 0;
    return $n
}

<STDIN> =~ /^Time: (.*)$/;
@times1 = split(' ', $1);
$time2 = $1;
$time2 =~ s/ //g;
<STDIN> =~ /^Distance: (.*)$/;
@distances1 = split(' ', $1);
$distance2 = $1;
$distance2 =~ s/ //g;
$len = scalar @times1;
die if ! $len == scalar @distances1;
$res1 = 1;
for ($i=0; $i<$len; $i++) {
    $res1 *= NumberOfWays(@times1[$i], @distances1[$i]);
}
$res2 = NumberOfWays($time2, $distance2);
print "$res1\n$res2\n";
