from collections import defaultdict
from heapq import heapify, heappush, heappop

class Twitter:

    def __init__(self):
        self.time = 0
        self.follows = defaultdict(set)
        self.tweets = defaultdict(list)    # min_heap -> (time, tweet)

    def postTweet(self, userId: int, tweetId: int) -> None:
        heappush(self.tweets[userId], (self.time, tweetId))
        self.time += 1

        if len(self.tweets[userId]) > 10:
            heappop(self.tweets[userId])


    def getNewsFeed(self, userId: int) -> List[int]:
        feed = self.tweets[userId][:]
        heapify(feed)

        for id in self.follows[userId]:
            for items in self.tweets[id]:
                heappush(feed, items)
                if len(feed) > 10:
                    heappop(feed)
        
        feed.sort(key=lambda x: x[0], reverse=True)
        return [tweetId for (_, tweetId) in feed]


    def follow(self, followerId: int, followeeId: int) -> None:
        if followerId == followeeId:
            return
        self.follows[followerId].add(followeeId)

    def unfollow(self, followerId: int, followeeId: int) -> None:
        if followerId == followeeId:
            return

        if followeeId in self.follows[followerId]:
            self.follows[followerId].remove(followeeId)
