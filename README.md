1.功能简介：POECenter实现了存证必要的基础功能，包括：创建存证、转移存证、撤销存证    

2.核心逻辑：pallets/poe，其中pallet/poe/src下的lib\mock\tests, 其中lib定义了存证模块的基本功能逻辑, mock用于定义模拟环境下运行时的配置，tests是存证功能的单元测试    

3.效果展示：  
* create_claim()用于创建存证。事件类型是ClaimCreated(),错误类型有：创建错误ProofAlreadyExist（凭证已存在）
* 单元测试方法create_claim_test()
* 创建存证的功能验证截图：
  <img width="1427" alt="创建凭证-1" src="https://github.com/lihuineo/POECenter/assets/161575076/4d2640fb-d1b2-48a3-8ed8-edac5c11a8ce">
  <img width="1438" alt="创建凭证-2" src="https://github.com/lihuineo/POECenter/assets/161575076/6d17b07a-25cc-41c3-a45f-f8614808ac63">
  <img width="1433" alt="创建凭证-3" src="https://github.com/lihuineo/POECenter/assets/161575076/cf813441-1f08-4fff-8e80-53c37394def2">

* revoke_claim()撤销存证。事件类型是ClaimRevoked()。错误类型有：ClaimNotExist（凭证不存在）, NotClaimOwner（没有撤销权限）  
* 单元测试方法撤销存证revoke_claim_test()
* 撤销存证的功能验证截图：
  <img width="1429" alt="删除凭证-1" src="https://github.com/lihuineo/POECenter/assets/161575076/3b69f1d9-1e7b-4484-a4eb-3037a2d5113a">
  <img width="1428" alt="删除凭证-2" src="https://github.com/lihuineo/POECenter/assets/161575076/dfb3ecb5-bcd3-4c14-a66f-4d5a535fc039">

* 转移存证transfer_claim()。对应的事件类型是ClaimTransferred()。错误类型有：NotNeedTransfer（重复转移）  
* 单元测试转移存证transfer_claim_test()  
* 转移存证的功能验证截图：
  <img width="1429" alt="转移凭证-1" src="https://github.com/lihuineo/POECenter/assets/161575076/e1496b60-8fb9-4ada-beb1-063dae33a846">
  <img width="1424" alt="转移凭证-2" src="https://github.com/lihuineo/POECenter/assets/161575076/0c2c6f20-6b4e-4b3a-ba6f-cb92746ea86c">
  <img width="1441" alt="转移凭证-3" src="https://github.com/lihuineo/POECenter/assets/161575076/9dc55440-2a09-43ca-84e7-ee2dca6045a2">



