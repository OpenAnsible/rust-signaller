Signaller Server
=====================

:Date: 05/16 2016

.. contents::


介绍
-------

`WebRTC` 信令服务, 主要用来帮助 `RTC Peer` 之间建立联系提供桥梁服务(类似于 Bittorrent Tracker Service)。

*信令服务是什么？*

    当我们初始化一个 `Peer` 实例后，我们就可以拿到 `自我描述` (SDP Offer)，然后我们需要把这份 `Offer` 通过其它的
    方式发送给另外一个 `Peer` ，另外一个 `Peer` 收到 `SDP Offer` 后会返回一个 `SDP Answer`，
    然后双方协商建立 `Peer To Peer` 的方式。那么承担这个任务的服务我们把它称为 `信令服务` 。 

    `WebRTC` 系列协议本身并没有对该功能做相应规范，一般来说我们可以直接采用 `WebSocket` 技术
    来实现 `信令服务` （如果浏览器已经支持了 `WebRTC` 的话，那显然也支持了 `WebSocket` ）。


*除了支持最基本的 `RTC Peer` 之间的 `SDP` 交换功能外，还支持哪些功能？*
    
    除了支持 `RTC Peer` 之间互相交换 `SDP` 之外，还支持了 `RTC Peer` 之间的互相聊天功能。
    这是考虑到直播的使用场景，所以我们可以把这个实时聊天功能应用在 `弹幕` 之上。
    在线聊天功能的消息格式参考了 `IRC` 协议的格式。

TODO
-------

1.   `Peers` 以及 `Channels` 数据存储在 `Redis` 当中，以应对程序崩溃、当机等灾难带来的内存数据丢失。


用例
-------

**Client:**

.. code:: javascript

    var socket = new WebSocket("ws://127.0.0.1:3012", []);
    socket.onmessage = function (event) {
        // server response message
        var message = event.data;
        console.log(message);
    };
    // descp ( Description your self, if not, the connection will be close. )
    socket.send("/descp {id: peer_id(bson.ObjectId), name: \"peer name\", ...}");

    // join(or create )
    socket.send("/join #channel_id(bson.ObjectId) with_channel_token");

    // quit
    socket.send("/quit #channel_id(bson.ObjectId)");

    // privmsg
    socket.send("/privmsg peer_id(bson.ObjectId) message_content");     // to peer

    // pubmsg
    socket.send("/pubmsg #channel_id(bson.ObjectId) message_content"); // to channel
    
    // broadcast
    socket.send("/broadcast message_content"); // to all connection (all channel, all peer)

    // whois
    socket.send("/whois peer_id(bson.ObjectId)");     // peer    info
    socket.send("/whois #channel_id(bson.ObjectId)"); // channel info

    // peers
    socket.send("/peers #channel_id(bson.ObjectId)");

    // invite
    socket.send("/invite peer_id(bson.ObjectId)");

    // away
    socket.send("/away");

    // kick
    socket.send("/kick peer_id(bson.ObjectId)");

    // ban
    socket.send("/ban peer_id(bson.ObjectId)");



参考
-------

*   `RFC 4566 <https://tools.ietf.org/html/rfc4566>`_ , WebRTC
*   `RFC 6455 <https://tools.ietf.org/html/rfc6455>`_ , WebSocket
*   `RFC 4566 <https://tools.ietf.org/html/rfc4566>`_ , Session Description Protocol (SDP)

