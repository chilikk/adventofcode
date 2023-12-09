use strict;
use vars qw/@vals $total1 $total2/;

sub Predict {
    my @vals = @_;
    my @diff = ();
    my $allsame = 1;
    my $prev = undef;
    for my $val (@vals) {
        if (defined($prev)) {
            push(@diff, ($val-$prev));
        }
        if ($val != $prev) {
            $prev = $val;
            $allsame = 0;
        }
    }
    return $prev if $allsame;
    return $prev + Predict(@diff);
}

while (<STDIN>) {
    @vals = split(' ', $_);
    $total1 += Predict(@vals);
    $total2 += Predict(reverse @vals);
}
print "$total1\n$total2\n";
