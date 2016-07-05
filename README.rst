Signaller Server
=====================

:Date: 05/16 2016

.. contents::


介绍
-------

`WebRTC` 信令服务, 主要用来帮助 `RTC Peer` 之间建立联系提供桥梁服务(类似于 Bittorrent Tracker Service)。

*信令服务是什么？* ::

    当我们初始化一个 `Peer` 实例后，我们就可以拿到 `自我描述` (SDP Offer)，然后我们需要把这份 `Offer` 通过其它的
    方式发送给另外一个 `Peer` ，另外一个 `Peer` 收到 `SDP Offer` 后会返回一个 `SDP Answer`，
    然后双方协商建立 `Peer To Peer` 的方式。那么承担这个任务的服务我们把它称为 `信令服务` 。 

    `WebRTC` 系列协议本身并没有对该功能做相应规范，
    一般来说我们可以直接采用 `WebSocket` 技术来实现 `信令服务` （如果浏览器已经支持了 `WebRTC` 的话，
    那显然也支持了 `WebSocket`）。


*除了支持最基本的 RTC Peer 之间的 SDP 互换功能外，还支持哪些功能？* ::
    
    除了支持 `RTC Peer` 之间互相交换 `SDP` 之外，还支持了 `RTC Peer` 之间的互相聊天功能。
    这是考虑到直播的使用场景，所以我们可以把这个实时聊天功能应用在 比如弹幕之上。
    在线聊天功能的消息格式参考了 `IRC` 协议的格式。


用例
-------

*   TODO


