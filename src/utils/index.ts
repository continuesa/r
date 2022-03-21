import EventEmitter from 'events';

// PC/移动端 REM 多端适配
interface IinitRemSize {
    switchFlag: boolean;
    callback: (gbcr: number) => void;
}
export const getRootRemSizeNew = ({ switchFlag, callback }: IinitRemSize): void => {
    function initResize() {
        // 默认的设计稿宽度 1920px
        let designSize = 1920;
        const html = document.documentElement;
        let gbcr: number = html.getBoundingClientRect().width;
        if (gbcr <= 750) {
            designSize = 750;
        }

        if (gbcr > 750 && gbcr <= 1440) {
            designSize = 1440;
        }

        if (gbcr > 1440 && gbcr <= 1920) {
            designSize = 1920;
        }

        if (gbcr > 1920) {
            designSize = gbcr;
        }

        callback(gbcr);

        if(switchFlag){
            const remSize = (gbcr / designSize) * 100;
            html.style.fontSize = `${remSize}px`;
        }else{
            // 如果小于或者等于 750 的设计图，就设置根元素字体大小来适配，否侧就不设置
            if (designSize <= 750) {
                const remSize = (gbcr / designSize) * 100;
                html.style.fontSize = `${remSize}px`;
            } else {
                html.style.fontSize = '';
            }
        }
        
    }

    initResize();

    window.addEventListener('resize', initResize, false);
  
};


// 获取滚动的参数  start
export interface IGetScrollTop {
    callback: ({initScrollVal, initStddecVal}: {initScrollVal: number, initStddecVal: number}) => void;
}

export interface IScroll {
    cab: ({initScrollVal, initStddecVal}: {initScrollVal: number, initStddecVal: number}) => void;
}
const getScrollTop = ({callback}: IGetScrollTop) => {
    const getScrollTopVal = document.documentElement.scrollTop;
    const initStddecVal = getScrollTopVal + document.documentElement.clientHeight;
    let initScrollVal;
    if (getScrollTopVal <= 20) {
        initScrollVal = 0;
    } else {
        initScrollVal = getScrollTopVal;
    }
    callback({initScrollVal, initStddecVal});
}
 
//获取文档内容的的高度
export const getScrollMethod = ({cab}: IScroll) => {
    window.addEventListener('scroll', () => {
        getScrollTop({
            callback: ({initScrollVal, initStddecVal}) => {
                cab({initScrollVal, initStddecVal});
            }
        });
    }, false);
}
// 获取滚动的参数  end

// 获取父元素的距离顶部的位置 start
export const getOffset = (curEle: any) => {
    let par = curEle.offsetParent, // 获取父级参照物
        l = curEle.offsetLeft, // 获取左边距离我的参照物的距离
        t = curEle.offsetTop; // 获取上边距离我的参照物的距离

    // 循环
    while (par && par.tagName !== 'BODY') {
        if (!/MSIE 8\./.test(navigator.userAgent)) {
            l += par.clientWidth;
            t += par.clientHeight;
        }
        l += par.offsetLeft;
        t += par.offsetTop;
        par = par.offsetParent;
    }

    return {
        top: t,
        left: l,
    }

};

export const getParentElement = (parentEle: any) => {
    return parentEle;
};
// 获取父元素的距离顶部的位置 end

// 获取当前元素距离顶部的值 start
export const getClientRect = (curEle:any, innerNum=0) => {
    let { bottom, height, left, right, top, width, x, y } = curEle.current.getBoundingClientRect(); // 当前元素
    const isClientRect = top >= 0 && bottom > window.innerHeight + innerNum;
    return {
        isClientRect,
        parms: {
            bottom, height, left, right, top, width, x, y
        }
     };
};
// 获取当前元素距离顶部的值 end

// 延时方法
export const delay = (ms: number, callback: Function): void => {
    setTimeout(() => {
        callback();
    }, ms);
};

//是移动端返回true,否则false
export const isMobile = (): boolean => {
    return (/phone|pad|pod|iPhone|iPod|ios|iPad|Android|Mobile|BlackBerry|IEMobile|MQQBrowser|JUC|Fennec|wOSBrowser|BrowserNG|WebOS|Symbian|Windows Phone|webOS|android/i.test(navigator.userAgent))
};

// 事件触发
/**
 *
 * @param currentEle    元素
 * @param eventMethod   事件方法
 * @param data          数据
 * @param callback      事件回调
 */

export interface IeventRun {
    currentEle: any;
    eventMethod: any;
    data: Object;
    callback: Function;
}
export const eventRun = ({currentEle, eventMethod, data, callback}: IeventRun) => {
    const event = new EventEmitter();
    const videoMessage = (msg: string): void => {
        callback({
            ele: currentEle, // 元素
            message: msg,    // 消息
        });
    };
    event.on(eventMethod, videoMessage);
    event.emit(eventMethod, data ? data : {});
}

// 发送邮箱 📪
export const mailtoSubmit = ({eMailAddress}: {eMailAddress: string}): void => {
    const who = prompt('请输入收件人邮箱: ', eMailAddress);
    const what = prompt('输入主题: ', 'Note');
    if (window.confirm(`你确定要向 ${who} 发送主题为 ${what} 的邮件么?`) === true) {
        window.parent.location.href = `mailto: ${who}?subject=${what} `;
    }
}

 //  获取指定指定时区时间（北京时区为8，纽约时区为-5。东时区为正数，西市区为负数）
 export const getSystemTime = (date: string) => {
    const timezone = 8;
   // 本地时间距离（GMT时间）毫秒数
   let nowDate = !date ? new Date().getTime() : new Date(date).getTime()
   // 本地时间和格林威治时间差，单位分钟
   let offset_GMT = new Date().getTimezoneOffset()
   //  反推到格林尼治时间
   let GMT = nowDate + offset_GMT * 60 * 1000 
   //  获取指定时区时间
   let targetDate = new Date(GMT + timezone * 60 * 60 * 1000);
   return targetDate
}