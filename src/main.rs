use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn main() {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let t_len =t.len();
        let s_chars:Vec<char> = s.chars().collect();
        let t_chars:Vec<char> = t.chars().collect();
        let mut  count_t_len = 0;
        for x  in s_chars.iter() {
            if x==&t_chars[count_t_len.clone()] {count_t_len +=1 }
            //println!("{}",count_t_len)

        }
        if count_t_len==t_len {
            return true;
        }
        false
    }
    pub fn is_palindrome(s:String) -> bool{
        let s_len = s.len();
        //let chars = s.to_lowercase().chars().collect::<Vec<char>>();
        let chars :Vec<char> = s.to_lowercase().chars().collect();
        let mut left= 0;
        let mut right = s_len-1 ;
        while left<right  {
            while !chars[left.clone()].is_alphabetic() {left+=1}
            while !chars[right.clone()].is_alphabetic() {right-=1}
            if left>=s_len||right<=0 {break}
            if chars[left.clone()]!=chars[right.clone()] {
                return false;
            }
            left +=1;
            right -=1;
        }
        true
    }
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut  left_point = 0;
        let mut  right_point = numbers.len()-1;

        while left_point < right_point {
            //if &numbers[left_point] + &numbers[right_point] { }
            if numbers[left_point]+numbers[right_point]>target {right_point-=1 }
            else if numbers[left_point]+numbers[right_point]<target {left_point+=1 }
            else { return vec![left_point.clone() as i32,right_point.clone() as i32] }
        }
        return vec![]

    }
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut dedup_sorted_nums = Vec::new();
        dedup_sorted_nums.push(nums[0].clone());
        for i in 1..nums.len() {
            if nums[i]==nums[i-1] {continue }
            dedup_sorted_nums.push(nums[i]);
        }
        let aa = dedup_sorted_nums.clone();
        println!("{:?}",aa);
        let len = dedup_sorted_nums.len();
        let mut result:Vec<Vec<i32>> = Vec::new();
        for i in 0..len - 1 {
            let mut l = i+1;
            let mut r = len - 1;
            while l < r {
                let sum = dedup_sorted_nums[i] + dedup_sorted_nums[l] + dedup_sorted_nums[r];
                if sum==0 {
                    result.push(vec![dedup_sorted_nums[i],dedup_sorted_nums[r],dedup_sorted_nums[l]]);
                    l+=1;
                    r-=1;
                }
                else if sum>0 { r-=1}
                else { l+=1 }
            }
        }
        return result

    }

    //使用滑动窗口
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 1;
        let mut max_dis = nums.len();
        while r < nums.len() {
            let mut sum = nums[l]+nums[r];
            while sum<target {
                r+=1;
                sum = sum + nums[r];
            }
            while sum>target {
                l+=1;
                sum = sum - nums[l-1];
            }
            if sum==target {
                max_dis = min(max_dis,r-l);
                r+=1;
            }
        }
        max_dis as i32
    }
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s_chars:Vec<char> = s.chars().collect();
        let mut l = 0;
        let mut r = 0;
        let mut count = 0;
        let mut set:HashSet<char> = HashSet::new();
        while r < s.len() {
            if !set.contains(&s_chars[r]) {
                set.insert(s_chars[r]);
                r+=1;
                //println!("{}",count);
                count = max(count,(r-l) as i32);
            }
            else {
                set.remove(&s_chars[l]);
                l=r
            }
        }
        return count as i32
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mag_chars:Vec<char> = magazine.chars().collect();
        let rans_chars:Vec<char> = ransom_note.chars().collect();
        let ransom_len = ransom_note.len();
        let mut count = 0;
        for i in 0..magazine.len() {
            if mag_chars[i]==rans_chars[count] {
                count=count+1;
            }
            if count==ransom_len {
                return true;
            }
        }
        return false
    }
    /*
    问题是，split(" ")返回一个迭代器，该迭代器产生&str类型的元素，而您试图将这些&str元素收集到Vec<String>中，这是不允许的，因为Rust不能自动将&str转换为String。
    */
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern_chars: Vec<char> = pattern.chars().collect();
        let s_strings: Vec<&str> = s.split(" ").collect();
        let mut map = HashMap::new();

        if pattern_chars.len() != s_strings.len() {
            return false; // 模式长度与字符串长度不匹配，返回false
        }

        for i in 0..pattern.len() {
            let res = map.insert(pattern_chars[i], s_strings[i]);
            if let Some(existing) = res {
                if existing != s_strings[i] {
                    return false; // 如果模式对应的值已存在且不相等，返回false
                }
            }
        }

        true
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        let s_chars:Vec<char> = s.chars().collect();
        let t_chars:Vec<char> = t.chars().collect();
        let mut s_map:HashMap<char,i32> = HashMap::new();
        let mut t_map:HashMap<char,i32> = HashMap::new();
        if s.len()!=t.len() {
            return false
        }
        for s_char in s_chars {
            if s_map.contains_key(&s_char) {
                s_map.insert(s_char,s_map.get(&s_char).unwrap()+ 1);
            }else {
                s_map.insert(s_char,1);
            }
        }
        for t_char in t_chars {
            if t_map.contains_key(&t_char) {
                t_map.insert(t_char,t_map.get(&t_char).unwrap()+1);
            }else {
                t_map.insert(t_char,1);
            }
        }
        for key in s_map.keys() {
            if s_map.get(key)!=t_map.get(key){
                return false;
            }
        }
        true
    }

    pub fn two_sums(mut nums: Vec<i32>,target: i32) ->Vec<i32>{
        nums.sort();
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            if target<nums[l]+nums[r] {
                l = l+1;
            }
            else if target>nums[l]+nums[r] {
                r = r -1;
            }
            else {
                break
            }
        }
        vec![l as i32,r as i32]
    }

    pub fn two_sum_index(nums: Vec<i32> ,target :i32) -> Vec<i32> {
        let mut map :HashMap<i32,i32> = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&(target - nums[i])) { return vec![*map.get(&(&target- nums[i])).unwrap(),i as i32]}
            map.insert(nums[i],i as i32);
        }
        return vec![-1,-1]
    }

    pub fn is_happy(num: i32) -> bool {
        let mut num = num;
        while num/10 > 0 {
            num = get_num_char(num);
            if num==1 { return true }
        }
        false
    }
    pub fn get_num_char(num: i32) -> i32 {
        let mut num = num;
        let mut new_num = 0;
        while num > 0 {
            let yu = num % 10;
            new_num = new_num + yu*yu;
            num = num / 10;
        }
        return new_num;
    }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map:HashMap<i32,i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(pre_index) = map.insert(num,i as i32) {
                if (i as i32 - pre_index) <= k {
                    return true
                }
            }
        }
        false
    }

    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut vec_num:Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            if vec_num.contains(&nums[i]) {
                continue
            }else {
                vec_num.push(nums[i]);
            }
        }
        let mut count = 1;
        let mut max_len = 1;
        for i in 1..vec_num.len() {
            if vec_num[i]==vec_num[i-1] +1 {
                count = count + 1;
            }
            else {
                count = 0;
            }
            max_len = max(max_len,count);
        }
        max_len
    }
    //汇总区间
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut nums_:Vec<i32> = Vec::new();
        let mut str: Vec<String> = Vec::new();
        let mut count = 0;
        for i in 0..nums.len() {
            nums_.push(nums[i] - i as i32);
        }
        for i in 0..nums_.len()-1 {
            let mut start = 0;
            let mut end = 0;
            if nums_[i+1]==nums_[i] {
                count = count + 1;
            }else {
                end = nums_[i] + i as i32;
                start = end - count;
                count = 0;
                let res = format!("{}->{}",start,end);
                str.push(res);
            }

        }
        str
    }
    //合并区间
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut inter:Vec<Vec<i32>> = Vec::new();
        let mut intervals = intervals;
        intervals.sort_by(|a,b| a[1].cmp(&b[1]));
        println!("{:?}",intervals);
        let mut new_vec= vec![intervals[0][0],intervals[0][1]];
        for i in 1..intervals.len() {
            if intervals[i][0]<=new_vec[1] {
                new_vec = vec![min(new_vec[0],intervals[i][0]),intervals[i][1]];
                if i==intervals.len()-1 {
                    inter.push(new_vec.clone());
                }
            }else {
                inter.push(new_vec.clone());
                new_vec = vec![intervals[i][0],intervals[i][1]]
            }
        }
        return inter
    }

    let intervals = vec![vec![1,2],vec![3,7],vec![4,5],vec![7,9]];
    let res = merge(intervals);
    println!("{:?}",res);

    let mut nums = vec![vec![1, 2], vec![2, 5], vec![3, 4]];

    nums.sort_by(|a, b| a[1].cmp(&b[1])); // 按第二个元素升序排序

    for item in &nums {
        println!("{:?}", item);
    }

    let nums = vec![1,2,3,5,6,9];
    let res = summary_ranges(nums);
    println!("{:?}",res);
    
    let nums = vec![1,2,3,4,7,8,9,5,6,100,101,7];
    let res = longest_consecutive(nums);
    println!("{}",res);

    let nums = vec![1,2,3,1];
    let res = contains_nearby_duplicate(nums,3);
    println!("{}",res);

    let res = is_happy(18);
    println!("{}-----",res);

    let nums = vec![3,2,1,6];
    let res = two_sum_index(nums,4);
    println!("{:?}",res);

    let nums = vec![1,2,3,4];
    let res = two_sums(nums,9);
    println!("{:?}",res);

    let s = String::from("abacd");
    let t = String::from("acbaw");
    let res = is_anagram(s,t);
    println!("{}",res);

    let pattern = String::from("aba");
    let magaine = String::from("hello word nihao");
    let res =  word_pattern(pattern,magaine);

    println!("{}====",res);

    let mut map = HashMap::new();

    // 插入键值对，如果键已存在，则返回false
    let result = map.insert("key1", "value1");
    if let Some(_) = result {
        println!("插入失败，键已存在");
    } else {
        println!("插入成功");
    }

    // 再次尝试插入相同的键值对，返回false
    let result = map.insert("key1", "new_value");
    if let Some(_) = result {
        println!("插入失败，键已存在");
    } else {
        println!("插入成功");
    }

    let mut map = HashMap::new();
    map.insert(1,"s");
    map.insert(1,"k");
    println!("{:?}++++++++++",map);

    let ransom_note = String::from("ase");
    let magazine = String::from("afsbfse");
    let res = can_construct(ransom_note,magazine);
    println!("{}",res);

    let s = String::from("helplou");
    let res = length_of_longest_substring(s);
    println!("{}",res);

    let nums = vec![2,1,4,3,9,4];
    let res = min_sub_array_len(7,nums);
    println!("{:?}",res);

    let numbers = vec![1,2,3,4];
    let target = 4;
    let res = two_sum(numbers,target);
    println!("{:?}",res);

    let s = String::from("hseh.;");
    let res = is_palindrome(s);
    println!("{}",res);

    let s = String::from("iqwertthyp");
    let t = String::from("qetp");
    let re = is_subsequence(s,t);
    println!("{}",re);

    let mut  nums = vec![-1,-4,7,-2,3,1,6];
    nums.sort();
    println!("{:?}",nums);
    let res = three_sum(nums);
    println!("{:?}",res);
    let max = i32::MAX;

    println!("{}",max)

}