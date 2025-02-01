<template>
  <div>
    <div>
      <h3>音乐节奏</h3>
      <el-form ref="form" :model="midiForm" :rules="midiFormRules" :inline="true"  label-width="120px">
        <el-form-item label="BPM" prop="bpm">
          <el-input
            v-model.number="midiForm.bpm"
            type="number"
            placeholder="录入midiBPM"
            :min="1"
          />
        </el-form-item>
        <el-form-item label="调整时长" prop="videoPlayTime">
          <el-input
            v-model="midiForm.videoPlayTime"
            type="text "
            placeholder="时长 格式： 00:01:57.03"
            :min="1"
          />
        </el-form-item>
      </el-form>
      

      <div id="music-rhythm" style="
            width: 100%;
            overflow-x: auto;
        ">
        <div 
          class="progress-bar" 
          @wheel="handleWheel"
          :style="{ width: progressBarWidth + 'px', height: progressBarHeight + 'px' }"
        >
          <div 
            v-for="(note, index) in musicRhythm" 
            :key="index" 
            class="note" 
            :style="{ width: noteWidth(note), left: notePosition(note), top: noteTop(note) }"
            @mouseenter="hoverNote(note.key, $event)"
            @mouseleave="hoverNote(null)"
          ></div>
        </div>
        <div v-if="hoveredKey !== null" class="hovered-key" :style="{ left: mouseX + 'px', top: mouseY + 'px' }">
          Key: {{ hoveredKey }}
        </div>
      </div>
    </div>
    <div>
      <h3>动画预览</h3>
      <div id="animation-preview">
      </div>
    </div>
    <div>
      <div class="image-grid">
        <div v-for="(image, index) in images" :key="index" class="image-upload">
          <div class="button-container">
            <el-upload
              class="upload-demo"
              action="#"
              :show-file-list="false"
              :auto-upload="false"
              :on-change="(file) => handleImageUpload(file, index)"
              accept="image/*"
            >
              <template v-slot:trigger>
                <el-button type="primary">选择图片</el-button>
              </template>
            </el-upload>
            <span class="delete-icon" @click="removeImage(index)">×</span>
          </div>
          <el-select v-model="image.condition" placeholder="出现条件" class="condition-select">
            <el-option label="大于" value="greater_than" />
            <el-option label="等于" value="equal_to" />
            <el-option label="小于" value="less_than" />
            <el-option label="范围" value="range" />
            <el-option label="默认留白" value="any_value" />
          </el-select>

          <el-input
            v-if="image.condition !== 'any_value'"
            v-model="image.value"
            :placeholder="getImageValuePlaceholder()"
            class="image-value-input"
            :disabled="image.condition === 'any_value'"
            :rules="getImageValueRules()"
            @input="validateInput(index)"
          />

          <div v-if="image.url" class="thumbnail">
            <img :src="image.url" alt="Thumbnail" />
          </div>
          <div v-if="validationErrors[index]" class="error-message">
            {{ validationErrors[index] }}
          </div>
        </div>
      </div>
      <div>
        <el-button type="primary" @click="addImageUpload">添加图片上传框</el-button>
      </div>
    </div>
    <div class="bottom-right-buttons">
      <el-upload
        class="upload-demo"
        action="#"
        :show-file-list="false"
        :auto-upload="false"
        :on-change="handleMidiUpload"
        accept=".mid,.midi"
      >
        <template v-slot:trigger>
          <el-button type="primary">选择MIDI文件</el-button>
        </template>
      </el-upload>
      <el-button type="primary" @click="selectOutputFolder">
        <div v-if="outputFilePath" class="selected-path" :title="outputFilePath">
          {{ "输出路径:" + outputFilePath }}
        </div>
        <span v-else>选择视频输出路径</span>
      </el-button>
      <el-button type="success" @click="generateAnimation" :disabled="isLoading" >{{ isLoading ? '生成中...' : '生成动画' }}</el-button>
    </div>
  </div>
</template>

<script>
import { ElButton, ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog'; // 引入 open 函数

export default {
  components: {
    ElButton,
  },
  data() {
    return {
      midiFile: null,
      images: [],
      musicRhythm: [],
      totalTime: 0,
      progressBarWidth: 1000, // 初始宽度
      progressBarHeight: 100, // 初始高度
      validationErrors: {}, // 新增：用于存储每个输入框的校验状态
      hoveredKey: null, // 新增：用于存储当前悬停的 note 的 key 值
      mouseX: 0,
      mouseY: 0,
      isLoading: false,
      midiForm:{
        bpm: 65,
        videoPlayTime: '00:00:00.00',
      },
      midiFormRules: {
        bpm: [
          { required: true, message: '请输入BPM', trigger: 'blur' },
          { type: 'number', message: 'BPM必须为数字值', trigger: 'blur' },
        ],
        videoPlayTime: [
          { required: true, message: '请输入视频播放时长', trigger: 'blur' },
          { validator: (rule, value, callback) => {
            // 校验时间格式 00:00：00.00
            if (!/^(\d{2}:){2}\d{2}.\d{2}$/.test(value)) {
              callback(new Error('时间格式不正确,请输入格式：00:00:00.00'));
            } else {
              callback();
            }   
          }, trigger: 'blur' }
        ]
      },
      outputFilePath: '', // 新增：用于存储输出文件路径
    };
  },
  methods: {
    // 处理 MIDI 文件上传
    async handleMidiUpload(file) {
      this.midiFile = file.raw;
      const arrayBuffer = await this.midiFile.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);
      const midiData = Array.from(uint8Array);

      try {
        const rhythm = await invoke('parse_midi', { file: midiData,bpm: this.midiForm.bpm });
        this.musicRhythm = JSON.parse(rhythm);
        this.calculateTotalTime();
      }  
      catch (error) {
        ElMessage({  message: `解析midi文件失败: ${ JSON.stringify(error) }`,  type: 'warning',}) 
      }
    },
    // 计算总时间
    calculateTotalTime() {
      if (this.musicRhythm.length > 0) {
        const lastNote = this.musicRhythm[this.musicRhythm.length - 1];
        this.totalTime = lastNote.start_time + lastNote.duration;
        this.progressBarWidth = this.musicRhythm.length * 50
      }
    },
    // 处理图片上传
    async handleImageUpload(file, index) {
        if (file) {
            this.images[index].file = file.raw;
            const reader = new FileReader();
            reader.onload = (e) => {
                this.images[index].url = e.target.result; // 直接赋值完整的 base64 数据 URL
            };
            reader.readAsDataURL(file.raw);
        }
    },
    // 添加图片上传框
    addImageUpload() {
      this.images.push({ id: '', file: null, url: '' });
    },
    // 生成动画
    async generateAnimation() {
      let res = await this.$refs.form.validate()
      
      console.log(res);
      // 校验midiForm 
      if (!this.midiFile) {
        ElMessage({  message: '请上传midi文件',  type: 'warning',})
        return;
      }
      if (this.images.length === 0) {
        ElMessage({  message: '请上传图片',  type: 'warning',})
        return;
      }
      if (!this.midiForm.videoPlayTime) {
        ElMessage({  message: '请输入视频播放时长',  type: 'warning',})
        return;
      }
      if (!this.outputFilePath) {
        ElMessage({  message: '请选择输出路径',  type: 'warning',})
        return;
      }
      let isValid = true;
      this.images.forEach((image, index) => {
        if (image.condition !== 'any_value') {
          const rules = this.getImageValueRules(image.condition);
          const value = image.value;
          let error = false;
          rules.forEach(rule => {
            if (rule.validator) {
              rule.validator(rule, value, (err) => {
                if (err) {
                  error = true;
                  this.validationErrors[index] = err.message; // 替换为直接赋值
                } else {
                  delete this.validationErrors[index]; // 替换为直接删除属性
                }
              });
            }
          });
          if (error) {
            isValid = false;
          }
        }
      });
      if (isValid) {
        this.isLoading = true;

        const arrayBuffer = await this.midiFile.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        const midiData = Array.from(uint8Array);

        // 根据上传的 MIDI 文件和图片生成动画
        invoke('generate_animation', { file: midiData, images: this.images , bpm: this.midiForm.bpm, videoPlayTime: this.midiForm.videoPlayTime, outputPath: this.outputFilePath })
          .then(response => {
            ElMessage({  message: '已生成动画',  type: 'success',})
          })
          .catch(error => {
            ElMessage({  message: `生成动画失败: ${error}`,  type: 'warning',})
          }).finally(()=>{
            this.isLoading = false;
          })
      }
    },
    // 删除图片
    removeImage(index) {
      this.images.splice(index, 1);
    },
    // 计算音符宽度
    noteWidth(note) {
      if (note.duration <= 0) {
        return '0.01%';
      }
      return `${Math.min((note.duration / this.totalTime) * 100, 100)}%`; // 确保宽度不超过100%
    },
    // 计算音符位置
    notePosition(note) {
      return `${(note.start_time / this.totalTime) * 100}%`;
    },
    // 计算音符纵向位置
    noteTop(note) {
      // MIDI 键值范围通常是 0 到 127
      const keyHeight = this.progressBarHeight / 128;
      return `${(127 - note.key) * keyHeight}px`;
    },
    // 处理鼠标滚轮事件
    handleWheel(event) {
      const delta = Math.sign(event.deltaY);
      if (event.ctrlKey) {
        this.progressBarWidth = Math.max(100, this.progressBarWidth - delta * 50); // 最小宽度为100px
      } else if (event.altKey) {
        this.progressBarHeight = Math.max(50, this.progressBarHeight - delta * 50); // 最小高度为50px
      }
    },
    hoverNote(key, event) {
      if (key !== null) {
        this.hoveredKey = key;
        this.mouseX = event.clientX;
        this.mouseY = event.clientY;
      } else {
        this.hoveredKey = null;
      }
    },
    getImageValuePlaceholder() {
      const condition = this.images.find(image => image.condition !== 'any_value')?.condition;
      switch (condition) {
        case 'greater_than':
          return '请输入一个大于条件的数字';
        case 'equal_to':
          return '请输入一个等于条件的数字';
        case 'less_than':
          return '请输入一个小于条件的数字';
        case 'range':
          return '请输入一个长度为2的数字数组，例如 [1, 5]';
        default:
          return '';
      }
    },
    getImageValueRules(condition) {
      let rules = [];
      switch (condition) {
        case 'greater_than':
        case 'equal_to':
        case 'less_than':
          rules.push({
            required: true,
            message: '请输入一个有效的数字',
            trigger: 'blur',
            validator: (rule, value, callback) => {
              if (!value || isNaN(value)) {
                callback(new Error('请输入一个有效的数字'));
              } else {
                callback();
              }
            }
          });
          break;
        case 'range':
          rules.push({
            required: true,
            message: '请输入一个长度为2的数字数组',
            trigger: 'blur',
            validator: (rule, value, callback) => {
              try{
                value = JSON.parse(value);
              } catch (e) {
                callback(new Error('请输入一个长度为2的数字数组'));
              }
              if (!Array.isArray(value) || value.length !== 2 || value.some(isNaN)) {
                callback(new Error('请输入一个长度为2的数字数组'));
              } else {
                callback();
              }
            }
          });
          break;
        default:
          break;
      }
      return rules;
    },
    validateInput(index) {
      const image = this.images[index];
      if (image.condition !== 'any_value') {
        const rules = this.getImageValueRules(image.condition);
        const value = image.value;
        let error = false;
        rules.forEach(rule => {
          if (rule.validator) {
            rule.validator(rule, value, (err) => {
              if (err) {
                error = true;
                this.validationErrors[index] = err.message;
              } else {
                delete this.validationErrors[index];
              }
            });
          }
        });
        if (!error) {
          delete this.validationErrors[index];
        }
      }
    },
    // 处理输出文件路径选择
    async selectOutputFolder() {
      try {
        const selected = await open({
          directory: true, // 设置为选择文件夹
        });
        if (selected) {
          this.outputFilePath = selected;
        }
      } catch (error) {
        ElMessage({ message: '选择文件夹失败', type: 'warning' });
      }
    },
  },
};
</script>

<style>
.image-upload {
  display: flex;
  flex-direction: column; /* 修改为列方向 */
  align-items: flex-start; /* 左对齐 */
  margin-bottom: 10px;
  padding:0.5em;
  border: 1px solid #ccc;
}

.image-upload > div {
  margin-bottom: 0.5em;
}

.button-container {
  display: flex;
  flex-direction: row; /* 按钮在同一行 */
  width: 100%;
  justify-content: space-between;
  align-items: flex-start;
}

.image-id-input {
  margin-top: 1em; /* 图片ID输入框移到新的一行 */
}

.image-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}

.thumbnail {
  width: 100px;
  height: 100px;
  overflow: hidden;
  margin-top: 10px;
  background: #ccc;
}

.thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.bottom-right-buttons {
  position: fixed;
  bottom: 20px;
  right: 20px;
  display: flex;
  flex-direction: row; /* 确保按钮在同一行 */
  gap: 10px;
}

.progress-bar {
  position: relative;
  background-color: #ddd;
  margin-top: 20px;
}

.note {
  position: absolute;
  height: 1%; /* 修改为固定高度 */
  background-color: green;
}

.delete-icon {
  cursor: pointer;
  font-size: 1.2em;
  padding: 0 1em;
  color: red;
}
.delete-icon:hover {
  background-color: #fff;
}

.image-value-input.error {
  border-color: red;
}

.error-message {
  color: red;
  font-size: 0.9em;
  margin-top: 5px;
}

.hovered-key {
  position: absolute;
  background-color: rgba(255, 255, 255, 0.8);
  border: 1px solid #000;
  padding: 5px;
  color: #000;
  border-radius: 5px;
  z-index: 1000;
}

.midi-info-item{
  width: 300px; 
  display: flex; 
  justify-content: space-between; 
  align-items: center;
  margin: 0.2em 0;
}

.midi-info-item > .el-input {
  width: 50%;
}

.selected-path {
  max-width: 100px;
  overflow: hidden;
}

</style>