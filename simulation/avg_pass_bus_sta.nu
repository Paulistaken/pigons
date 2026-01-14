#!/bin/nu

def get_avrg2 [res] {
    $res | reduce --fold [0,0] {|tm,acm| let ma = $tm + $acm.0; let mb = $acm.1 + 1; [$ma, $mb] }
}
def get_avrg [res] {
    $res | reduce --fold [0,0] {|tm,acm| let ma = $tm.io + $acm.0; let mb = $acm.1 + 1; [$ma, $mb] }
}
def get_avg_2 [ln] {
    mut res = $ln.items | update items {|ln| get_avg $ln }
    $res = $res | update items {|ln| $ln.items.0 / $ln.items.1}
    $res = $res | sort-by items | reverse
    $res = $res | rename hour avg
    $res
}
def get_avg [ln] {
    mut res = $ln.items | select io | flatten
    # $res = $res | reduce --fold [0,0] {|tm,acm| let ma = $tm.io + $acm.0; let mb = $acm.1 + 1; [$ma, $mb] }
    $res = get_avrg $res
    $res
}

def main [bus_no : string] {
    let path = "./simresults/pre/bus_logs/bus" + $bus_no + ".csv"

    mut res = open $path
    $res = $res | insert io {|ln| $ln.in + $ln.out } 
    $res = $res | group-by station --to-table
    $res = $res | update items {|ln| $ln.items | group-by hour --to-table }
    $res = $res | update items {|ln| get_avg_2 $ln } 
    $res = $res | sort-by -c {|a, b| $a.items.avg.0 > $b.items.avg.0 }
    $res = $res | insert avgt {|ln| get_avrg2 $ln.items.avg}
    $res = $res | update avgt {|ln| $ln.avgt.0 / $ln.avgt.1}
    $res = $res | update items {|ln| $ln.items | table -i false}
    $res = $res | rename "station id" "avg per hour" | table -i false
    return $res
}
