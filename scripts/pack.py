# TODO: 将__backend__的target目录下的编译结果
# 1. 根据架构和操作系统和输入名, 对编译结果进行重命名: {name}.{prefix}
# 2. 将重命名后的文件拷贝到__frontend__下的`Plugins`目录下, 根据{arch}和{os}确定目标位置;
# 3. 可选参数[-mode { (debug) | release }], 确定从哪里拷贝编译结果(debug or release);
# 4. 可选参数[=name { ("agent") | :string-input: }], 用于重命名目标文件;
# 5. 【重要】为每个目标生成Unity Meta信息;

# python ./scripts/pack.py [-name { ("agent") | :string: }] -[mode { (debug) | release }]