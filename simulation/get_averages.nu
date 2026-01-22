def main [] {
    open simresults/pre/bus_logs/bus1.csv | group-by station hour --to-table | insert sum_in {|ln| $ln.items.in | reduce --fold 0 {|a, b| $a + $b } } | insert sum_out {|ln| $ln.items.out | reduce --fold 0 {|a, b| $a + $b}} | insert amt {|ln| $ln.items.in | length} | select station hour sum_in sum_out amt | insert avg_in {|ln| $ln.sum_in / $ln.amt } | insert avg_out {|ln| $ln.sum_out / $ln.amt} | group-by station --to-table | update items {|ln| $ln.items | select hour avg_in avg_out}
}
