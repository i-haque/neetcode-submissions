use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

struct Twitter {
    time: i32,
    tweets: HashMap<i32, BinaryHeap<Reverse<(i32, i32)>>>,
    follows: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    pub fn new() -> Self {
        let time: i32 = 0;
        let tweets: HashMap<i32, BinaryHeap<Reverse<(i32, i32)>>> = HashMap::new();
        let follows: HashMap<i32, HashSet<i32>> = HashMap::new();

        Self {time, tweets, follows}
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let tweet_heap = self.tweets.entry(user_id).or_insert_with(BinaryHeap::new);
        tweet_heap.push(Reverse((self.time, tweet_id)));
        self.time += 1;

        if tweet_heap.len() > 10 {
            tweet_heap.pop();
        }
    }

    pub fn get_news_feed(&mut self, user_id: i32) -> Vec<i32> {
        let mut temp_feed: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        if let Some(tweet_heap) = self.tweets.get(&user_id) {
            for item in tweet_heap {
                temp_feed.push(*item);
            }
        }

        if let Some(user_set) = self.follows.get(&user_id) {
            for id in user_set {
                if let Some(tweet_heap) = self.tweets.get(&id) {
                    for item in tweet_heap {
                        temp_feed.push(*item);

                        if temp_feed.len() > 10 {
                            temp_feed.pop();
                        }
                    }

                }
            }
        }

        let mut feed: Vec<i32> = Vec::with_capacity(11);
        while let Some(Reverse((_, tweet_id))) = temp_feed.pop() {
            feed.push(tweet_id);
        }
        feed.reverse();

        feed
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            self.follows
                .entry(follower_id)
                .or_insert_with(HashSet::new)
                .insert(followee_id);
        }
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            if let Some(set) = self.follows.get_mut(&follower_id) {
                set.remove(&followee_id);
            }
        }
    }
}
