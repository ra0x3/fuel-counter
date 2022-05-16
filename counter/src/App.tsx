import React, {useState} from 'react';
import './App.css';
import {Flex, Heading, Button} from 'rebass';
import axios, {AxiosRequestConfig, AxiosResponse} from 'axios';

const config = {
  api_url: 'http://localhost:8000',
};

const api_axios = axios.create({
  baseURL: config.api_url,
});

namespace http {
  type GetCountResponse = {
    success: boolean;
    count: number;
  };

  type PostCountResponse = {
    success: boolean;
  };

  export async function get_count(): Promise<AxiosResponse<GetCountResponse>> {
    const conf: AxiosRequestConfig = {
      method: 'get',
      url: '/count',
    };
    const response = await api_axios(conf);
    return response.data;
  }

  export async function post_count(): Promise<AxiosResponse<PostCountResponse>> {
    const conf: AxiosRequestConfig = {
      method: 'post',
      url: '/count',
    };
    const response = await api_axios(conf);
    return response.data;
  }
}

let fake_count = 1;

function getContractCount(): number {
  fake_count++;
  return fake_count;
}

function App() {
  const [count, setCount] = useState(fake_count);
  return (
    <Flex
      flexDirection="column"
      justifyContent="center"
      alignItems="center"
      id="container"
      sx={{
        width: '100%',
        height: '100%',
        border: '1px solid red',
      }}
    >
      <Button
        variant="primary"
        sx={{marginBottom: 10}}
        onClick={() => {
          setCount(getContractCount());
        }}
      >
        Click Me
      </Button>
      <Heading>Count is {count}</Heading>
    </Flex>
  );
}

export default App;
