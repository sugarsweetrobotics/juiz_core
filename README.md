# juiz - the robot middleware

## モチベーション Motivation

まず、このソフトウェアを作った動機を書いておく。

このソフトウェアに興味を持った人ならROSの存在は知っていると思う。
ロボットのソフトウェアを作るためのデファクトスタンダードとなっているソフトウェア基盤だ。
私はこのROSについて大きな不満があるわけではない。
もちろん小さな不満はたくさんあって、集めれば大きな不満2つか3つ分にはなるだろうが、些細なことだ。
いまとりあえず実験したい、最新の研究をキャッチアップしたい、などのプロトタイピング目的でROSを使うのは賛成する。

一方で、ROSはバージョン2においても、古典的かつ曖昧なプログラミングモデルを引き続き許しており、ROSを使って継続的に発展するソフトウェア開発を行う場合はリスクが伴う。
大きなリスクとして、モジュールの大きさについて明確な定義や指針がないまま、ソフトウェアの状態をカプセル化している点である。
ROSに限らず、状態をカプセル化するソフトウェアモジュールは継続的発展を経るとモジュールそれぞれが肥大化する傾向があり、より多くの状態を包含してコードが複雑化する。
例えば、モジュール作成時に想定していなかった振る舞いの調整のために新たな引数が加わったり、状態調整のための設定値が加わる。
また、PUSH型のデータフロー型通信で設計したデータのやり取りが、データの送信の成功・失敗を受け取る要求が顕在化したため、結果を受け取るためのデータフロー通信を追加したり、冗長な遠隔呼び出し型通信を再定義したりする。
これを避けるためにSOLIDなどの開発指針を厳格に導入することが、ソフトウェア内部に加えてコンポーネント設計のレベルにおいても重要であるが、ロボットに関わる開発エンジニアチームのソフトウェアリテラシーを高く維持するのは難しい。

この問題の焦点は一重に、機能モジュールを提供する開発者が、最終的にユーザがどう使うか、を想定して機能モジュールのインターフェースを設計しなければいけない、という非常に困難な設計問題にある。
機能モジュールのユーザーのシステム開発が進んで、アプリケーションのサービスドメインに適用した場合、これはドメインによって多種多様であり、これを想定することは不可能だと筆者は考える。
一方で、現在普及しているROSやOpenRTM-aistは、機能モジュールの設計方法として冗長かつ柔軟性に欠けた設計がなされており、これらのモデルの延長線上で上記の問題を克服するのは難しいと考えている。

一方でウェブなど、ロボット開発と比較してソフトウェアの新陳代謝が活発な世界では、コンポーネントの可搬性はそのままに、内包する状態を複数のコンポーネントにまたがって集中管理するためのインフラ整備や、状態を積極的に交換するための共通APIを用意することが一般的になっている。
たとえば、ウェブフロントエンドでスタンダードの1つとなっているフレームワークのReactを例に挙げてみる。
Reactはキャンバスやボタン、テキストフィールドなどのHTML部品と、その動作を司るJavascriptないしはTypescriptのコードを一つのコンポーネントとしてまとめ、このコンポーネントを組み合わせてまた大きなコンポーネントや、ウェブページを構成するソフトウェアフレームワークである。
Reactでは内部状態をstateという変数を介してコンポーネント内部の状態の読み書きや自動更新を管理しているが、しばしばこの状態を上位のコンポーネントや、隣り合うコンポーネントとやり取りするために多くのコードを書く必要があった。
これに対して、複数の状態管理フレームワークが提案されている。
例えば、事実上の標準となっているReduxでは、Fluxアーキテクチャと呼ばれる、ソフトウェア全体の状態を集中管理しつつ、状態の読み書きのデータの流れを一方通行にすることで単純化している。
Metaが開発しているRecoilでは、グローバルなキーに紐付けられた状態を任意のタイミングで取得し、また同時に得られるアクセサーで更新することで状態をReduxに比べると分散的に管理している。

このように、ソフトウェア規模が大きくなるとコンポーネントが内包する状態を隠蔽しすぎることの弊害が指摘される一方で、ROSに見られるコンポーネント型かつ比較的厳格な直積型データ型を利用するアーキテクチャでは、心筋梗塞を起こしやすい。
ROSでは内部状態を表す変数を他のソフトウェアと共有するためには、Topic、ActionおよびServiceが利用される。
Topicはpush型のデータフロー型通信を提供する。送信側は受信側の状態等を意識する必要は無いが、受信側から制御するのは難しい。
Serviceは遠隔呼び出し型通信で、クライアント・サーバー的なデータ交換を提供する。
クライアント側からのサービスの呼び出しには引数を与えることが出来、結果としてデータを受信できる。
またActionはServiceの発展版と言える通信で、結果の受信まで時間の掛かる遠隔呼び出し通信において、途中経過をFeedbackと言う形で受け取ることが出来る。
またモジュール外部からデータを受け取る仕組みとしてparameterとdynamic configureが用意されている。
parameterは起動時に一度だけ渡すことができる設定値であり、サーボ制御のゲインや使うデバイスのファイル名など、更新頻度が小さいが、実行時に設定を渡すための方法として提供されている。
またdynamic configureはparameterの動的なバージョンとして捉えられているが、実際はtopicで提供されるデータ通信であり、プログラマーからは変数が自動的に変化しているようあん便利なインターフェースを提供する。
これらはすべてデータの交換を目的として居ながら複数の仕組みが提供されている点で冗長であり、設計時の選択が必要な点で複雑である。
また、真に必要な状態を更新することが目的であるのに、それにまつわる振る舞いの調整のための変数までコンポーネント内部の状態として内包しており、このモデルで作成された機能モジュールは常に肥大化した状態と言って良い。

ROSの話が続いたが、同様に利用可能なオープンなロボットミドルウェアとしては日本の産業技術総合研究所のOpenRTM-aistや、NVIDIAのIssac SDK、デンソーなどが主に開発しているORiNなどがある。
プロプライエタリなソフトウェアとしてはSoftbank Roboticsが提供するnaoqiがユーザーが多いかもしれない。同社のPepperやNAOで使われているソフトウェアプラットフォームである。

これから提案するアーキテクチャはこれらの問題を奇麗に解決する方法としてはほど遠いが、これらのテイストを一部適用する結果としてシンプルかつ持続的に拡大可能なインフラストラクチャを提供できる可能性がある。
従って本ソフトウェアはロボット用ソフトウェアの新たな可能性に資するものとして、ここで開発し誰でも閲覧利用可能な形として保存されるものとする。

## 提案するアーキテクチャ

本プロジェクトで提案するアーキテクチャには、今の所、名前は無い。
その特徴としては、ROSのNodeのようなコンポーネント的アーキテクチャを分解し、その核となる状態変数をまとめたものを「コンテナ」、振る舞いを「プロセス」としたことにある。

コンテナはC言語で言えば構造体である。
変数をまとめて一つの単位として見做すことが出来るようにしたものである。
変数を束ねてグループ化し、一つの単位として見做せる、ということは、ソフトウェアの見通しやすさとして重要な機能である。

プロセスは一つの関数であると言える。
プロセスは任意の個数の引数を取り、一つの値を出力する。

またコンテナに割り付けられたプロセスを「コンテナプロセス」と呼ぶことにする。
コンテナプロセスはオブジェクト指向言語で言うところのクラスのインスタンスメソッドである。
純粋なプロセスとの違いとして、最初の引数として、そのコンテナプロセスが結びつけられたコンテナの実体への参照が渡される。
参照がリードオンリーな参照であれば、リードオンリーコンテナプロセス、書き込みも可能ならばライトコンテナプロセスと呼ぶことにする。

プロセスは基本的にべき等な写像であり、テストし易いこと、コードの見通しが良いことがメリットとして上げられる。
ロボット等の物理的なエフェクターの利用を考えた本プロジェクトでは、システムの振る舞いの始まりや終わりには、上述のコンテナプロセスの出番が多いと考えられる。
コンテナはI/Oアクセスのためのハンドルの置き場であり、コンテナプロセスでioctlを呼び出して制御するのが一般的であろう。
またコンテナは実行の結果をシリアライゼーション抜きで保存することができるため、副作用を請け負う物置きであるとも言える。

ロボット要素のプログラマーは、このコンテナとコンテナプロセス、および純粋なプロセスを用意することで機能を提供する。
ロボットの専用SDKをラッピングする形で機能提供することが多いと思うが、機能を司るクラスのオブジェクトをコンテナに持たせ、そのAPIをそれぞれコンテナプロセスでラッピングするのが通常の使い方になる。

コンテナやプロセス（コンテナプロセスを含む）は、プログラムが実行されると実体化される。
実体化されたコンテナやプロセスは、後述するブローカーを通して、いくつかのAPIを提供する。
プロセスが提供するAPIとして最も重要なものはcallである。
callは遠隔呼び出しであり、プロセスの引数全てを送信すると、プロセスの結果を受け取ることができる。
プロセスにどんな引数があるか、などの情報はprofileというAPIで取得できる。

プロセス同士はconnectが可能である。
connectでは、プロセスの出力を別のプロセスの引数の一つに繋ぐことができる。
プロセスは実体化すると各引数にバッファを持つ。
connectがpush型の場合は前段のプロセスが出力を出した場合に出力はバッファに貯められ、その直後に後段のプロセスを励起して出力を計算する。
connectがpull型の場合は、後段のプロセスが励起された場合に、前段の出力が計算される。
プロセスの励起は本提案ではexecuteというAPIで提供される。
executeは上述のようにconnectされた複数のプロセスを使って、状態の更新や外界への働きかけをリクエストすることに相当する。
executeは引数を持たないAPIであり、励起されるプロセスは引数に割り当てられたバッファおよび、pull型のconnectionを使って引数のデータを収集する。
connectionが無い引数はバッファに残されたキャッシュを使うが、このキャッシュはプロセスが実体化する際に、デフォルトの値が割り振られる。
またこの値はプロセスが提供するAPIであるbindで変更することが出来る。
bindの名前からも分かるとおり、このAPIは引数の部分適用に相当する機能であり、関数の振る舞いを調整するコンフィグレーションのような機能を提供する事が出来る。

以上の通り、本提案では「コンテナ」と「プロセス（コンテナプロセス）」が機能要素を実装する方法である。
提供する機能をプロセスの入力（引数）と出力として定義し、また副作用をコンテナに格納することで、ROSで得られた、データフロー型通信、遠隔呼び出し通信、動作の調整（パラメータ）が全て利用できるようになる。
この事は、機能要素を設計するエンジニアの負担を軽減するのみでなく、機能要素の再利用性を大幅に向上する。

## 機能要素の利用方法に関して

機能要素が実体化されると外部に向けてAPIを提供することは既に説明した。
これを利用することによりロボット要素を利用したアプリケーションを作るのが通常の利用方法になる。
機能要素の外部向けAPIは対象とする言語にあわせてラッピングされており、SDKの形で提供される。
これは、対象とする言語の様々なソフトウェアから利用しやすい機能としてロボットを提供する、という設計哲学の表れである。

このようにSDKの形で通信をラッピングして、プログラマフレンドリーな形でAPIを提供するロボットミドルウェアとしてはnaoqiやORiNが挙げられる。
一方でROSやOpenRTM-aistは、機能要素を利用する場合も機能要素としてソフトウェアを用意することを前提とした設計が見られる。
例えばROSではServiceの機能をクライアントとして利用するには、ROSのNodeとしての基本機能を有している必要があった。
一方でnaoqiでは、機能要素であるALModuleを実行するブローカーに対してリクエスト・レスポンス型の通信を行い機能を利用するが、これをラッピングする各言語のライブラリがあり、これをALProxyと呼び、このALProxyを介して、例えばPythonのプログラムを書く事が出来る。
これは、naoqiの機能要素を利用するプログラマーにとって、naoqiの提供するAPIに関する知識が殆ど必要無いことを意味している。
また、ALModuleは実体化されるとドキュメントを自動生成し、ブローカーで動作するhttpサーバー上でドキュメントを閲覧出来るため、通常のライブラリとして提供される以上の知識を得る方法もまた標準化されている。

本提案でも、同様にProxyライブラリを提供することで、各プログラミング言語の任意のアプリケーションに組み込みやすい形での機能提供を考えている。
本提案が考えるシステム開発のモデルは、継続的に状態を更新し続ける処理、特にリアルタイム性が高い処理は機能要素をconnectして大きな機能要素を作り、キーとなるプロセスを周期的にexecuteすることで状態を更新し続ける。
一方で、ロボットが適用されるサービスのドメイン、例えば工場のアセンブリ工程や、農作物の収穫作業の自動化、自動走行する搬送機械などが挙げられるが、これらのロボットを統合して価値を生み出すソフトウェアを構築するためには、Proxyライブラリを使うことを想定している。

ちなみに脱線するが、juizの実装では、周期的にexecuteを呼ぶスレッドの作成が頻出パターンであったので、特別に「実行コンテキスト、ExecutionContext」の機能を提供している。
EC (Execution Context) は、実体化するとprocessを結びつけることができる。
またECはSTART_STATEおよびSTOP_STATEの状態を持っており、外部APIでECをstartしてSTART_STATEに遷移すると、processをexecuteする。
ECには種類があり、デフォルトで提供しているTimerECは、定められたrateに従ってSTART_STATEである間は周期的にprocessをexecuteする。
またデフォルトで提供されているMainLoopECは、OSがプログラムに割り当てたメインのスレッド上でprocessをexecuteすることができる。
これはmain threadでの実行を要求するOSおよび主にGUI等のライブラリの利用上で便利な機能となる。

一方で、ロボットやロボット要素を使う開発者は、Proxyライブラリを使って独自のアプリケーションを作る。
研究者であればmain関数でロボット要素を初期化するコマンドを送った後、ループ内で繰り返し、状態の取得とアクチュエータの動作を指令するプログラムを書くかもしれない。
特定のプロセスが励起された場合に呼ばれるコールバックを使ってイベントドリブンなアプリケーションを書くこともできる。
もちろん、ロボットを利用する側の開発者が機能要素を開発することも可能である。

このように本提案モデルでは、多層的なユーザー層を想定した、ユーザーとの接点の設計を行っている。
この設計はnaoqiに強く影響を受けている。
いずれはchoregraphのようなグラフィカルなツールを用意することを準備している。

## 実装について

上記のように本提案が提供するのは機能要素との通信機能を提供するミドルウェアと、それを利用するためのラッパーライブラリであるプロキシーである。

ミドルウェア部の実装はRust言語を用いたcrateとして実装されている。
主に、機能要素を開発するためのjuiz_sdkと、機能要素を実体化するためのツールとしてのjuiz_coreおよびjuiz_appである。

機能要素を提供するユーザーは、juiz_sdk crateを利用して機能要素を作成する。
機能要素のためのコードはスケルトンコードを自動生成するためのアプリケーションを開発中である。
これを使ってビルドしたコードはdynamic link library (DLL. .so, .dylib, .dllファイル) として提供できる。

機能要素を利用してシステムを構成するユーザは、juiz_appが提供するjuizコマンドを使う。
juizコマンドに、yaml形式の設定ファイルを読み込ませる。
このyaml形式ファイルがしてするDLLをjuizコマンドがロードし、設定ファイルに従ってコンテナやプロセスを実体化する。
コンテナやプロセスはCoreBrokerによって管理されており、CoreBrokerと外部APIとのインターフェースはBrokerと名付けられている。
BrokerはCoreBrokerを通してコンテナやプロセスにアクセスするためのAPIを定義したインターフェースである。
Brokerの実装として、デフォルトでHTTP+JSONとQUIC (バイナリ) が提供されている。
特にHTTPのBrokerはデフォルトでOpenAPIのインターフェース定義を提供するので、SwaggerUIで動作確認をすることが可能である。

設定ファイルの例について示す。
``` yaml
"name": "test_system"
"option":
  "http_broker":
    "start": true
    "port": 8000
"plugins":  
  "container_factories":
    "example_container":
      "language": "rust"
      "path": "./target/debug"
      "processes":
        "example_container_get":
          "path": "./target/debug"
        "example_container_increment":
          "path": "./target/debug"
  "process_factories":
    "increment_process":
      "path": "./target/debug"
"containers":
  - "type_name": "example_container"
    "name": "container0"
    "processes":
    - "type_name": "example_container_increment"
      "name": "increment0"
    - "type_name": "example_container_get"
      "name": "get0"
"processes":
  - "type_name": "increment_process"
    "name": "increment0" 
```
トップレベルの「name」はシステムの名前を定義する。

「option」はデフォルトで動作するモジュールの動作定義をする。
「http_broker」はhttp Brokerの振る舞いについて定義できる。
「start」をtrueにするとデフォルトでhttp_brokerが起動し、portで指定するポートで通信が可能になる。
これ以外にも後述するpythonpathなど、デフォルトの動作について調整できる。

「plugins」は、コンテナやプロセスおよびbrokerの実装のDLLを読み込むための定義が書かれている。
「container_factories」はコンテナのDLLの読み込み、「process_factories」はプロセスのDLL読み込みを行っている。

トップレベルの「containers」は、pluginsで読み込まれたコンテナを実体化するための設定が書かれている。
同様に「processes」は純粋プロセス実体化のための定義が書かれている。
