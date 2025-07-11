#[allow(dead_code)]
pub struct Solution;

// 1. Two Sum
impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();

        let len = nums.len();
        for i in 0..len {

            let req = target - nums[i];

            match map.get(&req) {
                Some(indx) => return vec![i as i32, *indx as i32],
                None => map.insert(nums[i], i as i32),
            };

        }

        return vec![];

    }

}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl ListNode {
    pub fn next_node(&self) -> &Option<Box<ListNode>> {
        &self.next
    }

    pub fn add_node(&mut self, val: i32) -> &mut Self {
        /* if let Some(node) = &mut self.next {
            node.add_node(val);
        }
        else {
            self.next = Some(Box::new(Self::new(val)));
        } */
        self.next = Some(Box::new(Self::new(val)));
        return self.next.as_mut().unwrap();
    }

    pub fn to_vec(&self) -> Vec<i32> {
        if self.next.is_none() { return vec![self.val]; }

        let mut vals = self.next.as_ref().unwrap().to_vec();
        let mut result = vec![self.val];
        result.append(&mut vals);

        return result;
    }
}

// 2. Add Two Numbers
impl Solution {

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut result: ListNode = ListNode::new(0);
        let mut last_node = &mut result;

        let (mut node_l1, mut node_l2) = (&l1, &l2);

        let mut carry = 0;

        loop {

            let mut dig_1 = 0;
            let mut dig_2 = 0;

            if let Some(node) = &node_l1 {
                dig_1 = node.val;
                node_l1 = &node.next;
            }

            if let Some(node) = &node_l2 {
                dig_2 = node.val;
                node_l2 = &node.next;
            }

            let res = dig_1 + dig_2 + carry;

            if res > 9 { carry = res / 10; }
            else { carry = 0; }

            last_node = last_node.add_node(res % 10);

            if node_l1.is_none() && node_l2.is_none() {
                if carry == 1 {
                    last_node.add_node(1);
                }
                break;
            }

        }

        //return Some(Box::new(result));
        return result.next;

    }

}

// 3. Length of longest substring
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
    
        let len = s.len();
        let s_bytes = s.as_bytes();

        use std::collections::HashMap;

        let mut i = 0;
        let mut non_repeat = 0;
        let mut longest = 0;
        let mut hash: HashMap<char, usize> = HashMap::new();
        
        loop {

            if i >= len { break; }

            let c = s_bytes[i] as char;

            match hash.get(&c) {

                Some(found_index) => {

                    i = found_index + 1;

                    hash.clear();

                    non_repeat = 0;

                }

                None => {

                    non_repeat += 1;
                    hash.insert(c, i);
                    i += 1;

                    longest = longest.max(non_repeat);

                }

            }

        }

        return longest as i32;

    }
}

// 4. Median of two sorted arrays
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        
        // merge the arrays while summing up the values

        let mut merged = Vec::new();
        let mut median;
        let len = nums1.len() + nums2.len();

        if len == 0 { return 0_f64; }

        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            }
            else {
                merged.push(nums2[j]);
                j += 1;
            }
        }

        if i >= nums1.len() {
            while j < nums2.len() {
                merged.push(nums2[j]);
                j += 1;
            }
        }
        else if j >= nums2.len() {
            while i < nums1.len() {
                merged.push(nums1[i]);
                i += 1;
            }
        }

        // pick the middle value if the length if odd
        // else the pick the middle two values

        if len % 2 != 0 {
            median = merged[(len as f64 / 2.0).ceil() as usize - 1] as f64;
        }
        else {
            median = merged[len / 2] as f64 + merged[len / 2 - 1] as f64;
            median /= 2.0;
        }

        return median;

    }
}

// 5. Longest palindrome from a string
impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        if s.len() == 1 { return s; }

        let mut longest: &str = "";

        let mut i;
        let mut j = 1;

        while j <= s.len() {

            i = 0;

            while i + j <= s.len() {
                let current = s[i..i+j].to_string();

                if current[0..1] == current[current.len()-1..current.len()] {
                    let current_rev: String = current.chars().rev().collect();
                    if current == current_rev { 
                        longest = &s[i..i+j];
                        break;
                    }
                }

                i += 1;
            }

            j += 1;

        }

        return longest.to_string();

    }
}

// 6. Zig-zag string
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {

        if num_rows == 1 || s.len() < num_rows as usize { return s; }

        let mut s_out = String::new();
        let mut s_list: Vec<String> = Vec::new();
        s_list.resize(num_rows as usize, String::new());
        let mut floor = 0;
        let mut go_down = true;

        for ch in s.chars() {
            s_list[floor].push(ch);
            if floor == 0 { go_down = true; } else if floor as i32 == num_rows-1 { go_down = false; }
            if go_down { floor += 1; } else { floor -= 1; }
        }

        for s_part in s_list {
            s_out += &s_part;
        }

        return s_out;
    }
}

// 7. Reverse a number
impl Solution {
    pub fn reverse(x: i32) -> i32 {

        let sign;

        if x < 0 { sign = -1; } else { sign = 1; }

        let x64 = x as i64;
        let s_str = x64.abs().to_string();

        let s_str_rev: String = s_str.chars().rev().collect();
        let x_rev = s_str_rev.parse::<i64>().unwrap() * sign;

        if x_rev < i32::MIN as i64 || x_rev > i32::MAX as i64 { return 0; }

        return x_rev as i32;

    }
}

// 8. Convert string to integer
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        
        let mut sign = 1;
        let mut res = 0i64;

        for (i, ch) in s.trim().chars().enumerate() {
            if i == 0 && ch == '-' { sign = -1; continue; }
            else if i == 0 && ch == '+' { sign = 1; continue; }
            match ch.to_digit(10) {
                Some(val) => {
                    res *= 10;
                    res += val as i64;
                },
                None => {
                    break;
                }
            }
            if res > i32::MAX as i64 + 1 { break; }
        }   

        res *= sign;

        if res < i32::MIN as i64 { res = i32::MIN as i64; }
        if res > i32::MAX as i64 { res = i32::MAX as i64; }

        return res as i32;

    }
}

// 9. Check if a number is palindrome
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        
        let num = x as i64;

        let num_str = num.to_string();
        println!("{num_str}");
        let num_str_rev: String = num_str.chars().rev().collect();

        if num_str == num_str_rev { return true; }
        else { return false; }

    }
}

// 11. Container with most water
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {

        let mut i = 0;
        let mut j = height.len() - 1;

        let mut m_area = 0;

        while i < j {

            let area = (j - i) as i32 * height[i].min(height[j]);
            println!("{} * {} = {}", (j - i), height[i].min(height[j]), area);

            m_area = area.max(m_area);

            if height[i] < height[j] { i += 1; }
            else { j -= 1; }

        }
        
        return m_area;

    }
}

// 12. Integer to Roman
impl Solution {
    pub fn int_to_roman(num: i32) -> String {

        return "".to_string();

    }
}
