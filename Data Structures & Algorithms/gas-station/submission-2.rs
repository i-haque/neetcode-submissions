impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();

        let (mut total_gas, mut total_cost) = (0, 0);
        for i in 0..n {
            total_gas += gas[i];
            total_cost += cost[i];
        }
        if total_cost > total_gas {
            return -1;
        }

        let mut start_index = 0;
        let mut expense = 0;
        for i in 0..n {
            expense += gas[i] - cost[i];
            if expense < 0 {
                expense = 0;
                start_index = (i+1) as i32;
            }
        }

        start_index
    }
}
