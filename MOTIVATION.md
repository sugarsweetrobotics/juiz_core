# juiz - the robot middleware

## モチベーション Motivation

まず、このソフトウェアを作った動機を書いておく。

このソフトウェアに興味を持った人ならROSの存在は知っていると思う。
ロボットのソフトウェアを作るためのデファクトスタンダードとなっているソフトウェア基盤だ。
私はこのROSについて大きな不満があるわけではない。
もちろん小さな不満はたくさんあって、集めれば大きな不満2つか3つ分にはなるだろうが、些細なことだ。
いまとりあえず実験したい、最新の研究をキャッチアップしたい、などのプロトタイピング目的でROSを使うのは賛成する。
むしろもっと多くの人にROSを使ってほしい。
最近はROS関連の書籍や入門サイトを作ってくれる人がたくさんいるので、僕は積極的にROSに関して発信するのはやめた。
良い書籍もたくさんあるので参照してほしい。

一方で、ROSはバージョン2においても、古典的かつ曖昧なプログラミングモデルを引き続き許しており、ROSを使って継続的に発展するソフトウェア開発を行う場合はリスクが伴う。
大きなリスクとして、モジュールの大きさについて明確な定義や指針がないまま、ソフトウェアの「状態 (State) 」をカプセル化している点である。
ROSに限らず、状態をカプセル化するソフトウェアモジュールは継続的発展を経るとモジュールそれぞれが肥大化する傾向があり、より多くの状態をカプセル内に包含してコードそのものが複雑化し、それを利用するインターフェースも複雑化する。
例えば、モジュール作成時に想定していなかった振る舞いの調整のために新たな引数が加わったり、状態調整のための設定値が加わるなどがあり、あるインターフェースを利用するために、別のインターフェースで事前に状態を変更しておくこと、などリモートで呼び出す際の暗黙的な規約などが加わるなどが起こる。
また、PUSH型のデータフロー型通信で設計したデータのやり取りが、データの送信の成功・失敗を受け取る要求が顕在化したため、結果を受け取るためのデータフロー通信を追加したり、冗長な遠隔呼び出し型通信を再定義したりする。
データフロー通信で送信後の状態遷移の結果を受け取るようになると、並行して別のプロセスからリクエストを行っている場合にデータレースが起きて問題が顕在化する。
このような問題は、ソフトウェア改修の前にシステム全体の設計について吟味しつつ設計を行うことで回避ができる。
例えば、SOLIDなどの開発指針を厳格に導入することが、ソフトウェア内部に加えてコンポーネント設計のレベルにおいても重要であるが、ロボットに関わる開発エンジニアチームのソフトウェアリテラシーを高く維持するのは難しい。

カプセル化によって内包するデータを逐次更新するための定期的なルーチン実行が隠蔽されることも大きな弊害となっている。
ROSではRate型オブジェクトを使って一定周期を保つようにsleepするのが一般的方法として取られた。
このモデルでは、周期処理に影響を与えるには、内部の自由変数にアクセスするしかない。
多くの場合は、Rateオブジェクトの周期を変更したり、文字列を与えて動作モードを変更したり、外部モジュールのDLLを指定して振る舞いを調整している。
この方法の大きな問題点は、設定する自由変数が周期的な処理のどの場面で消費されるのかが隠蔽されている点にある。
このため、処理の調整に自由度を持たせるため、自由変数を多数持つモジュールを設計した場合、こういった変数の説明に多大な労力が必要となる。
OpenRTM-aistでは周期実行されるべき処理をon_executeという関数にまとめ、これを実行コンテキストと呼ばれるオブジェクトから定期的に呼ぶモデルを実装している。
実行コンテキストは周期処理のリアルタイム性の調整や、イベントドリブンな実行をサポートする柔軟な手法である。
しかしながら、on_executeは引数として実行コンテキストのユニークなIDを取るのみであり、自由変数の処理箇所の隠蔽に関する問題は残る。

この問題の焦点は一重に、機能モジュールを提供する開発者が、最終的にユーザがどう使うか、を想定して機能モジュールのインターフェースを設計しなければいけない、という非常に困難な設計問題にある。
機能モジュール、例えば汎用型マニピュレータのソフトウェアを提供する時、そのソフトウェアモジュールの開発者は、そのユーザーのシステム開発が進み、アプリケーションのサービスドメインに適用した場合、ドメインによって多種多様な用途をあらかじめ想定することは不可能だと筆者は考える。
一方で、現在普及しているROSや、同様に機能モジュールをカプセル化して共通のインターフェースを提供するロボット用ミドルウェアであるOpenRTM-aistは、どちらも、機能モジュールの設計方法として冗長かつ柔軟性に欠けた設計がなされており、これらのモデルの延長線上で上記の問題を克服するのは難しいと筆者は考えている。
これを設計選択肢の冗長性の問題、と以下では呼称する。

一方でウェブなど、ロボット開発と比較してソフトウェアの新陳代謝が活発な世界では、コンポーネントの可搬性はそのままに、内包する状態を複数のコンポーネントにまたがって集中管理するためのインフラ整備や、状態を積極的に交換するための共通APIを用意することが一般的になっている。
たとえば、ウェブフロントエンドでスタンダードの1つとなっているフレームワークのReactを例に挙げてみる。
Reactはキャンバスやボタン、テキストフィールドなどのHTML部品と、その動作を司るJavascriptないしはTypescriptのコードを一つのコンポーネントとしてまとめ、このコンポーネントを組み合わせてまた大きなコンポーネントや、ウェブページを構成するソフトウェアフレームワークである。
Reactでは内部状態をstateという変数を介してコンポーネント内部の状態の読み書きや自動更新を管理しているが、しばしばこの状態を上位のコンポーネントや、隣り合うコンポーネントとやり取りするために多くのコードを書く必要があった。
これに対して、複数の状態管理フレームワークが提案されている。
例えば、事実上の標準となっているReduxでは、Fluxアーキテクチャと呼ばれる、ソフトウェア全体の状態を集中管理しつつ、状態の読み書きのデータの流れを一方通行にすることで単純化している。
Metaが開発しているRecoilでは、グローバルなキーに紐付けられた状態を任意のタイミングで取得し、また同時に得られるアクセサーで更新することで状態をReduxに比べると分散的に管理している。

このような状態の集中管理はデバッグのしやすさや、コンポーネントと外部とのインターフェースの設計困難問題に対して一定の評価ができるが、そもそもロボットミドルウェアには設計選択肢の冗長性の問題がある中で、新たな方法論を導入するのはミドルウェアの無意味な肥大化に繋がる。
またロボットが担うような機能安全等の問題が絡み合うミッションクリティカルな問題に対して、モジュール間の横串を指すようなソリューションは、システム全体のディペンダビリティを大きく損なう可能性がある。
そこで、筆者はここで新しい哲学を持ったミドルウェアの設計を提案することにする。

繰り返すが、ソフトウェア規模が大きくなるとコンポーネントが内包する状態を隠蔽しすぎることの弊害が指摘されており、ROSに見られるコンポーネント型かつ比較的厳格な直積型データ型を利用するアーキテクチャでは、心筋梗塞を起こしやすい。
ROSでは内部状態を表す変数を他のソフトウェアと共有するためには、Topic、ActionおよびServiceが利用される。
Topicはpush型のデータフロー型通信を提供する。送信側は受信側の状態等を意識する必要は無いが、受信側から制御するのは難しい。
Serviceは遠隔呼び出し型通信で、クライアント・サーバー的なデータ交換を提供する。
クライアント側からのサービスの呼び出しには引数を与えることが出来、結果としてデータを受信できる。
またActionはServiceの発展版と言える通信で、結果の受信まで時間の掛かる遠隔呼び出し通信において、途中経過をFeedbackと言う形で受け取ることが出来る。
またモジュール外部からデータを受け取る仕組みとしてparameterとdynamic configureが用意されている。
parameterは起動時に一度だけ渡すことができる設定値であり、サーボ制御のゲインや使うデバイスのファイル名など、更新頻度が小さいが、実行時に設定を渡すための方法として提供されている。
またdynamic configureはparameterの動的なバージョンとして捉えられているが、実際はtopicで提供されるデータ通信であり、プログラマーからは変数が自動的に変化しているように見える便利なインターフェースを提供する。
これらはすべてデータの交換を目的として居ながら複数の仕組みが提供されている点で冗長であり、設計時の選択が必要な点で複雑である。
内部の状態を外部と入出力する場合に、機能モジュールの設計者はこれらの中から一つ、ないしは複数を選択する必要がある点で困難である。
また、真に必要な状態を更新することが目的であるのに、それにまつわる振る舞いの調整のための変数までコンポーネント内部の変数として内包しており、このモデルで作成された機能モジュールは常に肥大化した状態と言って良い。

ROSの話が続いたが、同様に利用可能なオープンなロボットミドルウェアとしては日本の産業技術総合研究所のOpenRTM-aistや、NVIDIAのIssac SDK、デンソーなどが主に開発しているORiNなどがある。
プロプライエタリなソフトウェアとしてはSoftbank Roboticsが提供するnaoqiがユーザーが多いかもしれない。同社のPepperやNAOで使われているソフトウェアプラットフォームである。

これから提案するアーキテクチャはこれらの問題を奇麗に解決する方法としてはほど遠いが、これらのテイストを一部適用する結果としてシンプルかつ持続的に拡大可能なインフラストラクチャを提供できる可能性がある。
従って本ソフトウェアはロボット用ソフトウェアの新たな可能性に資するものとして、ここで開発し誰でも閲覧利用可能な形として保存されるものとする。

## 提案するアーキテクチャ

本プロジェクトで提案するアーキテクチャには、今の所、名前は無い。
その実装として、juizという名前をつけた。JUIZ （ジュイス）はポルトガル語で審判の意味だが、筆者が好きなアニメのキャラクターの名前から頂いた。
juizの特徴としては、ROSのNodeのようなコンポーネント的アーキテクチャを分解し、その核となる状態変数をまとめたものを「コンテナ」、振る舞いを「プロセス」としたことにある。
プロセスという名前には変遷があり、以前は国内の会議で「Operation」という名前で発表していたが、「データの処理をする」という意味でProcessという名前を選択した。
DockerコンテナやUnixプロセスと名前がかぶることもあり、これらの呼称については議論の余地があると考えている。
以下では単純にコンテナ、プロセスと呼ぶ場合は、juizにおけるコンテナ、プロセスの意と解釈してほしい。

コンテナはC言語で言えば構造体である。
変数をまとめて一つの単位として見做すことが出来るようにしたものである。
変数を束ねてグループ化し、一つの単位として見做せる、ということは、ソフトウェアの見通しやすさとして重要な機能である。

プロセスは一つの関数であると言える。
プロセスは任意の個数の引数を取り、一つの値を出力する。
プロセスに状態はなく、完全に冪等なサービスを提供することを前提としている。
プロセスに状態がなく、副作用がないという点で、プロセスは「関数」であると言っても良いと考えている。
これを後述するコンテナプロセスと区別する場合は、特別に「純粋プロセス」と呼ぶことにする。

純粋なプロセスだけではI/Oのアクセスが前提となるロボット用ミドルウェアの部品としては不十分であり、このためにコンテナに結びつけたプロセスである「コンテナプロセス」を定義する。
コンテナプロセスはオブジェクト指向言語で言うところのクラスのインスタンスメソッドである。
純粋なプロセスとの違いとして、最初の引数として、そのコンテナプロセスが結びつけられたコンテナの実体への参照が渡される。
参照がリードオンリーな参照であれば、リードコンテナプロセス、書き込みも可能ならばライトコンテナプロセスと呼ぶことにする。

純粋、コンテナに限らずプロセスは基本的にべき等な写像であり、テストし易いこと、コードの見通しが良いことがメリットとして上げられる。
ロボット等の物理的なエフェクターの利用を考えた本プロジェクトでは、システムの振る舞いの始まりや終わりには、上述のコンテナプロセスの出番が多いと考えられる。
コンテナの第一の役割はI/Oアクセスのためのファイルデスクプリタの置き場であり、コンテナプロセスでioctlを呼び出して制御するのが一般的な例である。
またコンテナは実行の結果をシリアライゼーション抜きで保存することができるため、副作用を高速に請け負う物置きであるとも言える。

ロボット要素のプログラマーは、このコンテナとコンテナプロセス、および純粋なプロセスを用意することで機能を提供する。
ロボットの専用SDKをラッピングする形で機能提供することが多いと思うが、機能を司るクラスのオブジェクトをコンテナに持たせ、そのAPIをそれぞれコンテナプロセスでラッピングするのが通常の使い方になる。

ここまで紹介したコンテナやプロセス（コンテナプロセスを含む）は型であり、プログラムが実行されると実体化される。
実体化されたコンテナやプロセスは、後述するブローカーを通して、いくつかのAPIを提供する。
プロセスが提供するAPIとして最も重要なものはcallである。
callは遠隔呼び出しであり、プロセスの引数全てを送信すると、プロセスの結果を受け取ることができる。
プロセスにどんな引数があるか、などの情報はprofileというAPIで取得できる。

call以外にもプロセスの処理を使う方法としてexecuteを提供している。
executeは引数がない遠隔呼び出しであり、この場合、プロセスは二つの方法から引数の値を得る。

その一つがconnectionである。
プロセス同士はconnectすることが可能である。
プロセスは実体化すると各引数および出力にバッファ (outlet, inlet) を持つが、connectでは、プロセスの出力 (outlet) を別のプロセスの引数の一つ (inlet) に繋ぐことができる。
connectionに繋がったプロセスのうち、出力する側をsourceとよび、入力を受ける側をdestinationと呼ぶ。
connectionにはタイプがある。
あるプロセスがexecuteされると、そのinletのうち、pull型connectionを持つものは、そのconnectionのsourceに対してexecuteを要求し、出力を受け取る。
プロセスの値を計算した後、outletの持つconnectionのうち、push型のものがあればそのdestinationに対してexecuteを要求する。
このようにconnectionではexecuteを伝播させることができる。

もう一つの方法が引数 (inlet) に与えられたバッファを使う方法である。
プロセスがexecuteされたときにinletにconnectionがない場合や、connectionがすべてpull型でない場合はキャッシュの値を引数に束縛する。
このキャッシュはプロセスが実体化する際に、デフォルトの値が割り振られ、またこの値はプロセスが提供するAPIであるp_applyで変更することが出来る。
p_applyの名前からも分かるとおり、このAPIは引数の部分適用 (partial apply) に相当する機能であり、関数の振る舞いを調整するコンフィグレーションのような機能を提供する事が出来る。

以上の通り、本提案では「コンテナ」と「プロセス（コンテナプロセス）」が機能要素を実装する方法である。
提供する機能をプロセスの入力（引数）と出力として定義し、また副作用をコンテナに格納することで、ROSで得られた、データフロー型通信、遠隔呼び出し通信、動作の調整（パラメータ）が全て利用できるようになる。
この事は、機能要素を設計するエンジニアの負担を軽減するのみでなく、機能要素の再利用性を大幅に向上する。

## 機能要素の利用方法に関して

機能要素が実体化されると外部に向けてAPIを提供することは既に説明した。
これを利用することによりロボット要素を利用したアプリケーションを作るのが通常の利用方法になる。
機能要素の外部向けAPIは対象とする言語にあわせてラッピングされており、SDKの形で提供される。
これは、対象とする言語の様々なソフトウェアから利用しやすい機能としてロボットを提供する、という設計哲学の表れである。

このようにSDKの形で通信をラッピングして、プログラマフレンドリーな形でAPIを提供するロボットミドルウェアとしてはnaoqiやORiNが挙げられる。
一方でROSやOpenRTM-aistは、機能要素を利用する場合も、そのユーザープログラムを機能要素として用意することを前提とした設計が見られる。
例えばROSではTopicやServiceの機能をクライアントとして利用するには、ROSのNodeとしての基本機能を有している必要があった。
一方でnaoqiでは、機能要素であるALModuleを実行するブローカーに対してリクエスト・レスポンス型の通信を行い機能を利用するが、これをラッピングする各言語のライブラリがあり、これをALProxyと呼び、このALProxyを介して、例えばPythonのプログラムを書く事が出来る。
これは、naoqiの機能要素を利用するプログラマーにとって、naoqiの提供するSDKやAPIに関する知識が殆ど必要無いことを意味している。
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
このyaml形式ファイルが指定するDLLをjuizコマンドがロードし、設定ファイルに従ってコンテナやプロセスを実体化する。
コンテナやプロセスはCoreBrokerによって管理されており、CoreBrokerと外部APIとのインターフェースはBrokerと名付けられている。
BrokerはCoreBrokerを通してコンテナやプロセスにアクセスするためのAPIを定義したインターフェースである。
Brokerの実装として、デフォルトでHTTP+JSONとQUIC (バイナリ) が提供されている。
特にHTTPのBrokerはデフォルトでOpenAPIのインターフェース定義を提供するので、SwaggerUIで動作確認をすることが可能である。

例えば
```
$ juiz -f examples/rust/container/example_container.conf -d 
```
のように、.confファイル (実際はyamlファイル) を-fオプションで利用する。-dオプションは実行後に待機するオプションで、Ctrl+Cでシグナルを送ると終了する。
juizコマンドが待機中は、デフォルトで8000番ポートでhttp_brokerが動作しており、提供するAPIをSwaggerUIで試すことができるので、
```
http://127.0.0.1:8000/docs
```
にアクセスすると動作する。

## 機能要素の実装方法
機能要素であるContainer, ContainerProcessおよびProcessは、Rust, Python, C++の３種類の言語で実装することができる。

### Processの実装
#### Rustでの実装

機能要素を実装するには、juiz_sdkというcrateを使う。Cargo.tomlは以下のようになる。
```toml
[package]
name = "increment_process"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
juiz_sdk = { path = "$PATH_TO/juiz_sdk/" }
```
$PATH_TOにはjuiz_sdk crateまでの相対パスを書く。 (これはcrates.ioにjuiz_sdkを登録したら楽になると思う。)

例えば、引数に1を足して返すだけの純粋プロセスのコードを書いてみる。

Rustで記述するのが現状ではもっともエレガントにProcessやContainerを記述できる。

``` rust
use juiz_sdk::prelude::*;

#[juiz_process]
fn increment_process(arg1: i64) -> JuizResult<Capsule> {
    log::trace!("increment_process({:?}) called", arg1);
    return Ok(jvalue!(arg1+1).into());
}
```
まずjuiz_sdk::prelude::*をインポートすると、基本的なマクロや変数の型が使えるようになる。
juiz_processマクロを当てた関数が、Processの本体になる。関数の名前がProcessのタイプ名になる。
引数は複数の引数が使えて、i64, f64, bool, String, Value, Vec<Value>などが使える。
引数の名前もパラメータになっている。
juiz_processマクロに引数を与えると、ドキュメントやデフォルト引数を自動生成できる。詳しい内容は後述（予定）

#### C++での実装

モジュールのローダーであるjuizコマンドはrustで書かれているが、他の言語とのインターフェースを持っているので、機能モジュールを別の言語で書くことができる。
C++では、exportすべき関数の名前と、扱うべきデータ型が決まっており、これを提供するヘッダーファイルであるjuiz.hが提供されている。
juiz.hはbindings/cppjuiz/includeディレクトリにあるので、このディレクトリにINCLDUE_PATHを通しておいてほしい。

``` c++
#include "juiz/juiz.h"

juiz::Value manifest() {
    return ProcessManifest{"increment_process_cpp"}
        .add_int_arg("arg1", "test_argument", 1)
        .into_value();
}

std::optional<int64_t> increment_process(juiz::CapsuleMap cm) {
    auto a = cm.get_int("arg1");
    return a + 1;
}

PROCESS_FACTORY(manifest, increment_process);
```
C++はRustで自動生成していた部分をかなり自分で書かないといけない。
これはいずれなんとかしたいが、できるのだろうか・・・

#### Pythonでの実装

PythonとのインターフェースはRustのPyO3 crateを用いて実装されており、入出力で扱うデータ型は主にintやstrなどのプリミティブやlist, tuple, dictなどの複合型になる。
独自のデータ型を使う場合は、dataclassを使って構成して、juizに渡す関数の出力ではasdictメソッドでdictに変換して送ることになる。

pythonはpyjuizというパッケージを作成してある。
bindings/pyjuizにPYTHONPATHを通しておくと便利だ。

``` python
from juiz import *

@juiz_process
def increment_process(arg1:int = 1):
    return arg1 + 1
```
Pythonはデコレータで記述量をかなり減らすことができた。

### Containerの実装
Containerはstructを与えてやることで実現する。
後述のContainerProcessはこのstructを最初の引数として受け取るProcessを定義することになる。

#### Rustでの実装
例によってRustでのContainerの記述はエレガントである。
Cargo.tomlについてはProcessの章を参照してほしい。

Containerを作成する関数にjuiz_containerのマクロアトリビュートを追加するだけで実現できる。
この関数をコンテナのコンストラクタと呼ぶことにする。
返り値はBoxして渡して欲しい。
``` rust
use juiz_sdk::prelude::*;

#[repr(Rust)]
pub struct ExampleContainer {
    pub value: i64
}

#[juiz_container]
fn example_container(initial_value: i64) -> JuizResult<Box<ExampleContainer>> {
    Ok(Box::new(ExampleContainer{value:initial_value}))
}
```

#### C++での実装
C++ではヘッダーファイル (*.h) でstructを定義して、ソースファイル (*.cpp) でコンストラクタ等を定義する。
ヘッダーファイルのPATHについてはProcessの章を参照してほしい。
``` c++ 
// -- example_contaienr.h
#pragma once

#include <cstdint>

class CppContainer {
public:
    int64_t value;
    CppContainer(int64_t v) : value(v) {}
};
```

コンテナのコンストラクタとしてcreate_container関数を定義している。
このあたりもtemplateを使えばもう少しウマく書けそうなんだけど、Rustとの接続の部分も含めて設計が必要で、難しい。
``` c++
// --- example_container_cpp.cpp
#include "juiz/juiz.h"
#include "example_container.h"

juiz::Value manifest() {
    return ContainerManifest("example_container_cpp").into_value();
}

CppContainer* create_container(juiz::Value value) {
    int64_t int_value = 0;
    if (value.isObjectValue()) {
        if (value.hasKey("value")) {
            auto objv = value.objectValue();
            auto v = objv["value"];
            if (v.isIntValue()) {
               int_value = v.intValue();
            }
        }   
    }
    return new CppContainer(int_value);
}

bool destroy_container(CppContainer* p_container) {
    if (p_container) {
        delete p_container;
        return true;
    }
    return false;
}

CONTAINER_FACTORY(manifest, create_container, destroy_container);
```
#### Pythonでの実装

Pythonはやはり記述としては少ないが、もう少しスッキリさせるにはデコレータでなんとかしたいと考えている。
pyjuizのPATHについてはProcessの章を参照してほしい。
juiz_containerデコレータで記述がスッキリした。
``` python

from juiz import *

class PyContainer:
    value: int
    def __init__(self, value):
        self.value = value

@juiz_container
def example_container_python(initial_value:int = 0):
    # print(f'example_container_python(value = {initial_value}) called')
    return PyContainer(initial_value)

```

### Container Processの実装

#### Rustでの実装
コンテナプロセスではjuiz_container_processマクロを使い、このマクロの引数に「container_type = ほにゃらら」という値を入れる。
マクロの引数は初めて出てきたが、実はjuiz_processやjuiz_containerにも引数を与えることができる。
Cargo.tomlについてはProcessの章を参照してほしい。
``` rust
use example_container::ExampleContainer;
use juiz_sdk::prelude::*;

#[juiz_container_process( container_type = "example_container" )]
fn increment_function(container: &mut ContainerImpl<ExampleContainer>, arg1: i64) -> JuizResult<Capsule> {
    container.value = container.value + arg1;
    return Ok(jvalue!(container.value).into());
}
```

#### C++での実装
C++はやはりどこか冗長な記述になってしまう。
コンテナを生成するコードで使ったヘッダーを再利用することで、同じ構造体にアクセスするコンテナプロセスを作ることができる。
juiz.hについてはProcessの章を参照してほしい。
``` c++
#include "juiz/juiz.h"
#include "example_container.h"

juiz::Value manifest() {
    return ProcessManifest("example_container_cpp_increment")
        .container_type("example_container_cpp")
        .add_int_arg("arg0", "test_argument", 2)
        .into_value();
}

std::optional<int64_t> example_container_increment(CppContainer* container, juiz::CapsuleMap cm) {
    int64_t v = cm.get_int("arg0");
    container->value = container->value + v;
    return container->value;
}

CONTAINER_PROCESS_FACTORY(CppContainer, manifest, example_container_increment)
```
#### Pythonでの実装
コンテナプロセスはやはりC++よりはスッキリと書ける。


``` python
from juiz import juiz_container_process

@juiz_container_process(
    container_type="example_container_python"
)
def example_container_python_get(container):
    # print(f'example_container_python_get({container}) called')
    return container.value
```

## 機能要素の単体実行方法
juizクレートをビルドするとjuizコマンドが生成される。このコマンドを使う。

### Processを試す。
たとえば、RustでつくったProcessがtarget/debug/libtalker.dylibだった場合、以下のコマンドで単体プロセスが実行できる。
```terminal
juiz --process target/debug/libtalker.dylib -l rust -e
```
--processオプションで生成物を指定する。Pythonなら.pyファイル、C++ならば.dllや.so, .dylibなどのバイナリである。
-lオプションは言語を指定する。rust|cpp|pythonの3つから選び、デフォルトはrustであるので例の場合は"-l rust"は省略が可能なオプションである。
-eオプションはロードしたプロセスを一つ、自動で名前をつけて実体化し、デフォルトの引数を使ってexecuteする。

このjuizクレートの中で試すなら、以下のように行う
```terminal
cargo run -- --process ./target/debug/libtalker.dylib -e
```
デフォルトのbinとしてjuizコマンドが登録されているのでcargo runで実行される。

-dオプションをつけると、実行後にサーバーを立ててそのまま待機をする。
この状態ならばhttp://localhost:8000/docsをブラウザで開くと、swagger-uiを使ったテストを行うことができる。
これについては後述する。

### ContainerおよびContainerProcessを試す。
Rustで作ったContainerとContainerProcessがそれぞれ、./target/debug/my_container.dylibと./target/debug/my_container_process.dylibであった場合、以下のコマンドで単体のコンテナプロセスを実行できる。
```terminal
juiz --container ./target/debug/my_container.dylib --container_process ./target/debug/my_container_process.dylib -l rust -e 
```
--containerでコンテナのファイルを、--container_processでコンテナプロセスのバイナリを指定する。
-lで言語を指定するのも同じであり、-eオプションも同じ効果である。

## 設定ファイルの中身
複数の成果物を一気に読み込む場合は設定ファイルを記述するのが簡単である。

設定ファイルの例について示す。
``` yaml
name: "test_system"
option:
  http_broker:
    start: true
    port: 8000
plugins:  
  container_factories:
    example_container:
      language: "rust"
      path: "./target/debug"
      processes:
        example_container_get:
          path: "./target/debug"
        example_container_increment:
          path: "./target/debug"
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
    "name": "inc0" 
```
現状、かなり記述量が多いので、これを減らすことを考えている。

このファイルではコンテナのファクトリーとして、example_container型のコンテナのファクトリーを含んだexample_container.dylibファイルと、そのexample_containerに結び付けられたexample_container_get型のコンテナプロセスのファクトリーを含んだdylibファイル、同じくexample_container下のexample_container_incrementのdylib、
同時に、純粋プロセスのファクトリーとしてincrement_processのdylibを読み込んでいる。
ファクトリーのdylibファイルは、それが提供する型の名前＋拡張子、で指定するルールになっている。
また、containerの実体としてexample_container型のcontainer0を実体化し、このコンテナのメンバーとしてexample_container_increment型のコンテナプロセスであるincrement0と、example_container_getのget0を実体化している。
また、純粋プロセスであるincrement_process型のinc0も実体化している。

ここでもう少し設定ファイルについて説明する。

トップレベルの「name」はシステムの名前を定義する。

「option」はデフォルトで動作するモジュールの動作定義をする。
「http_broker」はhttp Brokerの振る舞いについて定義できる。
「start」をtrueにするとデフォルトでhttp_brokerが起動し、portで指定するポートで通信が可能になる。
これ以外にも後述するpythonpathなど、デフォルトの動作について調整できる。

「plugins」は、コンテナやプロセスおよびbrokerの実装のDLLを読み込むための定義が書かれている。
「container_factories」はコンテナのDLLの読み込み、「process_factories」はプロセスのDLL読み込みを行っている。

トップレベルの「containers」は、pluginsで読み込まれたコンテナを実体化するための設定が書かれている。
同様に「processes」は純粋プロセス実体化のための定義が書かれている。


