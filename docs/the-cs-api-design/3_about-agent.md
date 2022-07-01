## 关于Transport

`Transport`代表IC网络对象, 一般使用url进行初始化, 主网url是: __https://ic0.app__;

```cs
// 创建IC主网网络对象
var transportMain = new Transport();

// 创建IC测试网网络对象
var transportTest = new Transport("http://127.0.0.1:8080");
```

一般方法(查询):

```cs
// 查询网络对象的目标;
var url = transport.GetUrl();

// 修改网络对象目标;
transport.SetUrl("https://ic0.app");

// 查询网络对象的状态:
//  【阻塞】
var statusSync = transport.GetStatusSync();
//  【非阻塞】待定
var statusAsync = await transport.GetStatusAsync();
```

## About Agent

```cs
// 创建Agent对象
Agent agent = new Agent(icNet = ?, identity = ?, ingressExpiry = ?);

// 重设url
agent.SetUrl(icNetUrl);

// Map function
var rootKey = agent.FetchRootKey();
agent.SetRootKey(..);
agent.GetPrincipal();


```